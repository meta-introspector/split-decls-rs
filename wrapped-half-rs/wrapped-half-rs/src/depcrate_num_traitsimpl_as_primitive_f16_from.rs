// Generated macro for impl_as_primitive_f16_from (macro)
macro_rules! Depcrate_num_traitsimpl_as_primitive_f16_from {
() => {
// Module: crate::num_traits
// Provides: {"impl_as_primitive_f16_from"}
// Dependencies: {}
macro_rules ! impl_as_primitive_f16_from { ($ ty : ty , $ meth : ident) => { impl AsPrimitive < f16 > for $ ty { # [inline] fn as_ (self) -> f16 { f16 ::$ meth (self . as_ ()) } } } ; }
};
}
