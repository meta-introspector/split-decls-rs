// Generated macro for impl_209 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_209 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_209"}
// Dependencies: {}
impl CheckRaw for CStr { type RawUnit = NonZeroChar ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { NonZeroChar :: new (c) . ok_or (EscapeError :: NulInCStr) } }
};
}
