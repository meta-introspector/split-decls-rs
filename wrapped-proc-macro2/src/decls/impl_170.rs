macro_rules! deps {
    () => {
        MixedUnit!();
        NonZeroChar!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl From < NonZeroU8 > for MixedUnit { # [inline] fn from (byte : NonZeroU8) -> Self { if byte . get () . is_ascii () { MixedUnit :: Char (NonZeroChar :: new (byte . get () as char) . unwrap ()) } else { MixedUnit :: HighByte (byte) } } }
    };
}

impl_170!();