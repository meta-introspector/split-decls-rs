// Generated macro for impl_42 (impl)
macro_rules! Depcrate_ast_dataimpl_42 {
() => {
// Module: crate::ast::data
// Provides: {"impl_42"}
// Dependencies: {}
impl < T : UsesTypeParams > UsesTypeParams for Fields < T > { fn uses_type_params < 'a > (& self , options : & usage :: Options , type_set : & 'a IdentSet ,) -> IdentRefSet < 'a > { self . fields . uses_type_params (options , type_set) } }
};
}
