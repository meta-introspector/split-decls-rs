macro_rules! deps {
    () => {
        VisibilityExplicitness!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl VisibilityExplicitness { pub fn is_explicit (& self) -> bool { matches ! (self , Self :: Explicit) } }
    };
}

impl_180!();