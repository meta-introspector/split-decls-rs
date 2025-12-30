// Generated macro for impl_225 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_225 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_225"}
// Dependencies: {}
impl Unescape for str { type Unit = char ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Ok ('\0') ; # [inline] fn nonzero_byte2unit (b : NonZeroU8) -> Self :: Unit { b . get () . into () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { Ok (c) } # [inline] fn hex2unit (b : u8) -> Result < Self :: Unit , EscapeError > { if b . is_ascii () { Ok (b as char) } else { Err (EscapeError :: OutOfRangeHexEscape) } } # [inline] fn unicode2unit (r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { r } }
};
}
