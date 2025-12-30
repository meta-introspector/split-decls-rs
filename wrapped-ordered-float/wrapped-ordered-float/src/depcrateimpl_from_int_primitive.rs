// Generated macro for impl_from_int_primitive (macro)
macro_rules! Depcrateimpl_from_int_primitive {
() => {
// Module: crate
// Provides: {"impl_from_int_primitive"}
// Dependencies: {}
macro_rules ! impl_from_int_primitive { ($ primitive : ty , $ inner : ty) => { impl From <$ primitive > for NotNan <$ inner > { fn from (source : $ primitive) -> Self { NotNan (<$ inner as From <$ primitive >>:: from (source)) } } } ; }
};
}
