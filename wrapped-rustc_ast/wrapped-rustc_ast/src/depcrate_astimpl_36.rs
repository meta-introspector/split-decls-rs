// Generated macro for impl_36 (impl)
macro_rules! Depcrate_astimpl_36 {
() => {
// Module: crate::ast
// Provides: {"impl_36"}
// Dependencies: {}
impl PathSegment { pub fn from_ident (ident : Ident) -> Self { PathSegment { ident , id : DUMMY_NODE_ID , args : None } } pub fn path_root (span : Span) -> Self { PathSegment :: from_ident (Ident :: new (kw :: PathRoot , span)) } pub fn span (& self) -> Span { match & self . args { Some (args) => self . ident . span . to (args . span ()) , None => self . ident . span , } } }
};
}
