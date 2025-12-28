macro_rules! deps {
    () => {
        CrateDisplayName!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl ops :: Deref for CrateDisplayName { type Target = Symbol ; fn deref (& self) -> & Symbol { & self . crate_name } }
    };
}

impl_32!();