// Generated macro for impl_217 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_217 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_217"}
// Dependencies: {}
impl From < NonZeroU8 > for MixedUnit { # [inline] fn from (byte : NonZeroU8) -> Self { if byte . get () . is_ascii () { MixedUnit :: Char (NonZeroChar :: new (byte . get () as char) . unwrap ()) } else { MixedUnit :: HighByte (byte) } } }
};
}
