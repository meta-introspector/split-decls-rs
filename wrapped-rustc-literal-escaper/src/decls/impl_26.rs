macro_rules! deps {
    () => {
        MixedUnit!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < NonZero < char > > for MixedUnit { # [inline] fn from (c : NonZero < char >) -> Self { MixedUnit :: Char (c) } }
    };
}

impl_26!()