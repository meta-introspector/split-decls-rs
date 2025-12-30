// Generated macro for impl_484 (impl)
macro_rules! Depcrate_resolverimpl_484 {
() => {
// Module: crate::resolver
// Provides: {"impl_484"}
// Dependencies: {}
impl < R1 : Resolver , R2 : Resolver > Resolver for (R1 , R2) { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > { self . 0 . resolve (arg) . or_else (| | self . 1 . resolve (arg)) } }
};
}
