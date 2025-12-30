// Generated macro for impl_227 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_227 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_227"}
// Dependencies: {}
impl Unescape for CStr { type Unit = MixedUnit ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Err (EscapeError :: NulInCStr) ; # [inline] fn nonzero_byte2unit (b : NonZeroU8) -> Self :: Unit { b . into () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { c . try_into () } # [inline] fn hex2unit (byte : u8) -> Result < Self :: Unit , EscapeError > { byte . try_into () } # [inline] fn unicode2unit (r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { Self :: char2unit (r ?) } }
};
}
