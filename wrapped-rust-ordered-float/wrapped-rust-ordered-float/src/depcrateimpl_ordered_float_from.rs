// Generated macro for impl_ordered_float_from (macro)
macro_rules! Depcrateimpl_ordered_float_from {
() => {
// Module: crate
// Provides: {"impl_ordered_float_from"}
// Dependencies: {}
macro_rules ! impl_ordered_float_from { ($ dst : ty , $ src : ty) => { impl From <$ src > for OrderedFloat <$ dst > { fn from (val : $ src) -> Self { OrderedFloat (val . into ()) } } } ; }
};
}
