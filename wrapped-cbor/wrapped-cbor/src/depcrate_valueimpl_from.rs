// Generated macro for impl_from (macro)
macro_rules! Depcrate_valueimpl_from {
() => {
// Module: crate::value
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ variant : path , $ for_type : ty) => { impl From <$ for_type > for Value { fn from (v : $ for_type) -> Value { $ variant (v . into ()) } } } ; }
};
}
