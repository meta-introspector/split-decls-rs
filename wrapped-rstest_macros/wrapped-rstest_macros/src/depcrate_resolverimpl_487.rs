// Generated macro for impl_487 (impl)
macro_rules! Depcrate_resolverimpl_487 {
() => {
// Module: crate::resolver
// Provides: {"impl_487"}
// Dependencies: {}
impl Resolver for (Pat , Expr) { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { if arg == & self . 0 { Some (Cow :: Borrowed (& self . 1)) } else { None } } }
};
}
