macro_rules! deps {
    () => {
        EscapeError!();
        MixedUnit!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl TryFrom < u8 > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (byte : u8) -> Result < Self , EscapeError > { NonZero :: new (byte) . map (From :: from) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_29!();