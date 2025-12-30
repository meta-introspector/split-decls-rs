// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ast_dataimpl_43 {
() => {
// Module: crate::ast::data
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : UsesLifetimes > UsesLifetimes for Fields < T > { fn uses_lifetimes < 'a > (& self , options : & usage :: Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { self . fields . uses_lifetimes (options , lifetimes) } }
};
}
