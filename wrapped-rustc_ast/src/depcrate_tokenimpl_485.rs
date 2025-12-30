// Generated macro for impl_485 (impl)
macro_rules! Depcrate_tokenimpl_485 {
() => {
// Module: crate::token
// Provides: {"impl_485"}
// Dependencies: {}
impl IdentIsRaw { pub fn to_print_mode_ident (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawIdent , } } pub fn to_print_mode_lifetime (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawLifetime , } } }
};
}
