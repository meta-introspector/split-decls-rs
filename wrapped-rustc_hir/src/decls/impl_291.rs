macro_rules! deps {
    () => {
        IsAsync!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl IsAsync { pub fn is_async (self) -> bool { matches ! (self , IsAsync :: Async (_)) } }
    };
}

impl_291!();