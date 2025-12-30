// Generated macro for impl_from (macro)
macro_rules! Depcrate_macrosimpl_from {
() => {
// Module: crate::macros
// Provides: {"impl_from"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! impl_from { ($ from : path , $ to : expr) => { impl From <$ from > for ErrorKind { fn from (e : $ from) -> Self { $ to (e) } } } ; }
};
}
