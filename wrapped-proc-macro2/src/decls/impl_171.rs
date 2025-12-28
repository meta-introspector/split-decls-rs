macro_rules! deps {
    () => {
        EscapeError!();
        NonZeroChar!();
        MixedUnit!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZeroChar :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_171!();