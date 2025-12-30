// Generated macro for impl_191 (impl)
macro_rules! Depcrate_deriving_coerce_pointeeimpl_191 {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for AlwaysErrorOnGenericParam < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } }
};
}
