macro_rules! deps {
    () => {
        NonZeroChar!();
        MixedUnit!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl From < NonZeroChar > for MixedUnit { # [inline] fn from (c : NonZeroChar) -> Self { MixedUnit :: Char (c) } }
    };
}

impl_169!()