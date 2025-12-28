macro_rules! deps {
    () => {
        EscapeError!();
        Unescape!();
        MixedUnit!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Unescape for CStr { type Unit = MixedUnit ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Err (EscapeError :: NulInCStr) ; # [inline] fn nonzero_byte2unit (b : NonZero < u8 >) -> Self :: Unit { b . into () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { c . try_into () } # [inline] fn hex2unit (byte : u8) -> Result < Self :: Unit , EscapeError > { byte . try_into () } # [inline] fn unicode2unit (r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { Self :: char2unit (r ?) } }
    };
}

impl_28!()