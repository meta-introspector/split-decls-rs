// Generated macro for impl_630 (impl)
macro_rules! Depcrate_ir_dynamic_typeimpl_630 {
() => {
// Module: crate::ir::dynamic_type
// Provides: {"impl_630"}
// Dependencies: {}
impl DynamicTypeData { # [doc = " Create a new dynamic type."] pub fn new (base_vector_ty : Type , dynamic_scale : GlobalValue) -> Self { assert ! (base_vector_ty . is_vector ()) ; Self { base_vector_ty , dynamic_scale , } } # [doc = " Convert 'base_vector_ty' into a concrete dynamic vector type."] pub fn concrete (& self) -> Option < Type > { self . base_vector_ty . vector_to_dynamic () } }
};
}
