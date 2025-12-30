// Generated macro for impl_from (macro)
macro_rules! Depcrate_contentimpl_from {
() => {
// Module: crate::content
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ ty : ty , $ newty : ident) => { impl From <$ ty > for Content { fn from (value : $ ty) -> Content { Content ::$ newty (value) } } } ; }
};
}
