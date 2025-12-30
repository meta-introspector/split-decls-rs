// Generated macro for impl_90 (impl)
macro_rules! Depcrate_syntax_kindimpl_90 {
() => {
// Module: crate::syntax_kind
// Provides: {"impl_90"}
// Dependencies: {}
impl From < u16 > for SyntaxKind { # [inline] fn from (d : u16) -> SyntaxKind { assert ! (d <= (SyntaxKind :: __LAST as u16)) ; unsafe { std :: mem :: transmute :: < u16 , SyntaxKind > (d) } } }
};
}
