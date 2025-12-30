// Generated macro for impl_as_primitive_to_f16 (macro)
macro_rules! Depcrate_num_traitsimpl_as_primitive_to_f16 {
() => {
// Module: crate::num_traits
// Provides: {"impl_as_primitive_to_f16"}
// Dependencies: {}
macro_rules ! impl_as_primitive_to_f16 { ($ ty : ty , $ meth : ident) => { impl AsPrimitive <$ ty > for f16 { # [inline] fn as_ (self) -> $ ty { self .$ meth () . as_ () } } } ; }
};
}
