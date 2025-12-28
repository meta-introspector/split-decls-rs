macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < u32 > for Token { fn from (token : u32) -> Self { Token (token) } }
    };
}

impl_8!()