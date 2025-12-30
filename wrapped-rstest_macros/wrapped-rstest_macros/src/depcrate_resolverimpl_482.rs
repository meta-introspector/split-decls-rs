// Generated macro for impl_482 (impl)
macro_rules! Depcrate_resolverimpl_482 {
() => {
// Module: crate::resolver
// Provides: {"impl_482"}
// Dependencies: {}
impl Resolver for HashMap < Pat , & '_ Expr > { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { self . get (arg) . or_else (| | self . get (& pat_invert_mutability (arg))) . map (| & c | Cow :: Borrowed (c)) } }
};
}
