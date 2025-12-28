macro_rules! deps {
    () => {
        Entry32!();
        Entry!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < Entry32 > for Entry { fn from (entry32 : Entry32) -> Self { entry32 . 0 } }
    };
}

impl_30!();