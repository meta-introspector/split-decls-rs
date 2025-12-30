// Generated macro for impl_483 (impl)
macro_rules! Depcrate_resolverimpl_483 {
() => {
// Module: crate::resolver
// Provides: {"impl_483"}
// Dependencies: {}
impl Resolver for HashMap < Pat , Expr > { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { self . get (arg) . map (Cow :: Borrowed) } }
};
}
