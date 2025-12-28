macro_rules! deps {
    () => {
        TokenId!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl TokenId { pub fn raw (self) -> usize { self . 0 } }
    };
}

impl_423!();