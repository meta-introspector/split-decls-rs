// Generated macro for impl_218 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_218 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_218"}
// Dependencies: {}
impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZeroChar :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }
};
}
