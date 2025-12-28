macro_rules! deps {
    () => {
        EscapeError!();
        MixedUnit!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZero :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_19!()