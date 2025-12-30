// Generated macro for impl_41 (impl)
macro_rules! Depcrate_ast_dataimpl_41 {
() => {
// Module: crate::ast::data
// Provides: {"impl_41"}
// Dependencies: {}
impl < T , U : Into < Vec < T > > > From < (Style , U) > for Fields < T > { fn from ((style , fields) : (Style , U)) -> Self { style . with_fields (fields) } }
};
}
