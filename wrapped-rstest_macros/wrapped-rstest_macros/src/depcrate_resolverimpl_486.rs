// Generated macro for impl_486 (impl)
macro_rules! Depcrate_resolverimpl_486 {
() => {
// Module: crate::resolver
// Provides: {"impl_486"}
// Dependencies: {}
impl < R : Resolver + ? Sized > Resolver for Box < R > { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { (* * self) . resolve (arg) } }
};
}
