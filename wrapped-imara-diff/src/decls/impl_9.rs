macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < Token > for u32 { fn from (token : Token) -> Self { token . 0 } }
    };
}

impl_9!();