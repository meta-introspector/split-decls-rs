macro_rules! deps {
    () => {
        MixedUnit!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < NonZero < char > > for MixedUnit { # [inline] fn from (c : NonZero < char >) -> Self { MixedUnit :: Char (c) } }
    };
}

impl_17!()