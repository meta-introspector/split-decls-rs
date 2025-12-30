// Generated macro for impl_226 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_226 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_226"}
// Dependencies: {}
impl Unescape for [u8] { type Unit = u8 ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Ok (b'\0') ; # [inline] fn nonzero_byte2unit (b : NonZeroU8) -> Self :: Unit { b . get () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { char2byte (c) } # [inline] fn hex2unit (b : u8) -> Result < Self :: Unit , EscapeError > { Ok (b) } # [inline] fn unicode2unit (_r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { Err (EscapeError :: UnicodeEscapeInByte) } }
};
}
