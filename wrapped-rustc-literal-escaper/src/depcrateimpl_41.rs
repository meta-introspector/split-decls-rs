// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Unescape for str { type Unit = char ; const ZERO_RESULT : Result < Self :: Unit , EscapeError > = Ok ('\0') ; # [inline] fn nonzero_byte2unit (b : NonZero < u8 >) -> Self :: Unit { b . get () . into () } # [inline] fn char2unit (c : char) -> Result < Self :: Unit , EscapeError > { Ok (c) } # [inline] fn hex2unit (b : u8) -> Result < Self :: Unit , EscapeError > { if b . is_ascii () { Ok (b as char) } else { Err (EscapeError :: OutOfRangeHexEscape) } } # [inline] fn unicode2unit (r : Result < char , EscapeError >) -> Result < Self :: Unit , EscapeError > { r } }
};
}
