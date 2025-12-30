// Generated macro for impl_as_primitive_bf16_from (macro)
macro_rules! Depcrate_num_traitsimpl_as_primitive_bf16_from {
() => {
// Module: crate::num_traits
// Provides: {"impl_as_primitive_bf16_from"}
// Dependencies: {}
macro_rules ! impl_as_primitive_bf16_from { ($ ty : ty , $ meth : ident) => { impl AsPrimitive < bf16 > for $ ty { # [inline] fn as_ (self) -> bf16 { bf16 ::$ meth (self . as_ ()) } } } ; }
};
}
