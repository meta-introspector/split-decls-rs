// Generated macro for impl_31 (impl)
macro_rules! Depcrate_ast_dataimpl_31 {
() => {
// Module: crate::ast::data
// Provides: {"impl_31"}
// Dependencies: {}
impl < V : UsesTypeParams , F : UsesTypeParams > UsesTypeParams for Data < V , F > { fn uses_type_params < 'a > (& self , options : & usage :: Options , type_set : & 'a IdentSet ,) -> IdentRefSet < 'a > { match * self { Data :: Struct (ref v) => v . uses_type_params (options , type_set) , Data :: Enum (ref v) => v . uses_type_params (options , type_set) , } } }
};
}
