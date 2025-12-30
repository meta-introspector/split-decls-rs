// Generated macro for impl_40 (impl)
macro_rules! Depcrate_astimpl_40 {
() => {
// Module: crate::ast
// Provides: {"impl_40"}
// Dependencies: {}
impl GenericArg { pub fn span (& self) -> Span { match self { GenericArg :: Lifetime (lt) => lt . ident . span , GenericArg :: Type (ty) => ty . span , GenericArg :: Const (ct) => ct . value . span , } } }
};
}
