// Generated macro for impl_171 (impl)
macro_rules! Depcrate_astimpl_171 {
() => {
// Module: crate::ast
// Provides: {"impl_171"}
// Dependencies: {}
impl InlineAsmTemplatePiece { # [doc = " Rebuilds the asm template string from its pieces."] pub fn to_string (s : & [Self]) -> String { use fmt :: Write ; let mut out = String :: new () ; for p in s . iter () { let _ = write ! (out , "{p}") ; } out } }
};
}
