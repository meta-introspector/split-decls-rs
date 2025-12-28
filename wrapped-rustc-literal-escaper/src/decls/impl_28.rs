macro_rules! deps {
    () => {
        MixedUnit!();
        EscapeError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZero :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_28!();