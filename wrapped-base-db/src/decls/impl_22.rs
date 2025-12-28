macro_rules! deps {
    () => {
        CrateName!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl ops :: Deref for CrateName { type Target = Symbol ; fn deref (& self) -> & Symbol { & self . 0 } }
    };
}

impl_22!()