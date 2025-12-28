macro_rules! deps {
    () => {
        TargetGround!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl TargetGround { # [inline] pub const fn code (& self) -> u8 { match self { Self :: Foreground => 30 , Self :: Background => 40 , } } }
    };
}

impl_70!();