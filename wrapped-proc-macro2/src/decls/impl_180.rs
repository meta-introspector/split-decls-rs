macro_rules! deps {
    () => {
        Unescape!();
        EscapeError!();
        MixedUnit!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Unescape for CStr { type Unit = MixedUnit ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Err (EscapeError :: NulInCStr) ; # [inline] fn nonzero_byte2unit (b : NonZeroU8) -> Self :: Unit { b . into () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { c . try_into () } # [inline] fn hex2unit (byte : u8) -> Result < Self :: Unit , EscapeError > { byte . try_into () } # [inline] fn unicode2unit (r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { Self :: char2unit (r ?) } }
    };
}

impl_180!()