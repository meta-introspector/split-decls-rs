// Generated macro for impl_32 (impl)
macro_rules! Depcrate_ast_dataimpl_32 {
() => {
// Module: crate::ast::data
// Provides: {"impl_32"}
// Dependencies: {}
impl < V : UsesLifetimes , F : UsesLifetimes > UsesLifetimes for Data < V , F > { fn uses_lifetimes < 'a > (& self , options : & usage :: Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { Data :: Struct (ref v) => v . uses_lifetimes (options , lifetimes) , Data :: Enum (ref v) => v . uses_lifetimes (options , lifetimes) , } } }
};
}
