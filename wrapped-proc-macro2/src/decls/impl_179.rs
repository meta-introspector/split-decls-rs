macro_rules! deps {
    () => {
        Unescape!();
        EscapeError!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl Unescape for [u8] { type Unit = u8 ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Ok (b'\0') ; # [inline] fn nonzero_byte2unit (b : NonZeroU8) -> Self :: Unit { b . get () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { char2byte (c) } # [inline] fn hex2unit (b : u8) -> Result < Self :: Unit , EscapeError > { Ok (b) } # [inline] fn unicode2unit (_r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { Err (EscapeError :: UnicodeEscapeInByte) } }
    };
}

impl_179!();