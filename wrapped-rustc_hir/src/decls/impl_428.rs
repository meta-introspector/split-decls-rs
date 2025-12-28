macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl Div < usize > for Limit { type Output = Limit ; fn div (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 / rhs) } }
    };
}

impl_428!()