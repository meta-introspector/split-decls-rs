macro_rules! deps {
    () => {
        MixedUnit!();
        EscapeError!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl TryFrom < u8 > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (byte : u8) -> Result < Self , EscapeError > { NonZeroU8 :: new (byte) . map (From :: from) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_172!()