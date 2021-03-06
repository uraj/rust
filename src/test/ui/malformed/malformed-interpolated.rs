#![feature(custom_attribute)]

macro_rules! check {
    ($expr: expr) => (
        #[my_attr = $expr] //~ ERROR unexpected token: `-0`
                           //~| ERROR unexpected token: `0 + 0`
        use main as _;
    );
}

check!("0"); // OK
check!(0); // OK
check!(0u8); //~ ERROR suffixed literals are not allowed in attributes
check!(-0); // ERROR, see above
check!(0 + 0); // ERROR, see above

fn main() {}
