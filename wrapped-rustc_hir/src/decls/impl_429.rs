macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl Mul < usize > for Limit { type Output = Limit ; fn mul (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 * rhs) } }
    };
}

impl_429!()