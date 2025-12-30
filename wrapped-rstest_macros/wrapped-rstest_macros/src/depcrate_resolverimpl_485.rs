// Generated macro for impl_485 (impl)
macro_rules! Depcrate_resolverimpl_485 {
() => {
// Module: crate::resolver
// Provides: {"impl_485"}
// Dependencies: {}
impl < R : Resolver + ? Sized > Resolver for & R { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { (* self) . resolve (arg) } }
};
}
