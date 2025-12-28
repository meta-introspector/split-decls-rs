macro_rules! deps {
    () => {
        Safety!();
        HeaderSafety!();
        Constness!();
        IsAsync!();
        FnHeader!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl FnHeader { pub fn is_async (& self) -> bool { matches ! (self . asyncness , IsAsync :: Async (_)) } pub fn is_const (& self) -> bool { matches ! (self . constness , Constness :: Const) } pub fn is_unsafe (& self) -> bool { self . safety () . is_unsafe () } pub fn is_safe (& self) -> bool { self . safety () . is_safe () } pub fn safety (& self) -> Safety { match self . safety { HeaderSafety :: SafeTargetFeatures => Safety :: Unsafe , HeaderSafety :: Normal (safety) => safety , } } }
    };
}

impl_321!();