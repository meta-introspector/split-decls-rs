// Generated macro for impl_219 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_219 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_219"}
// Dependencies: {}
impl TryFrom < u8 > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (byte : u8) -> Result < Self , EscapeError > { NonZeroU8 :: new (byte) . map (From :: from) . ok_or (EscapeError :: NulInCStr) } }
};
}
