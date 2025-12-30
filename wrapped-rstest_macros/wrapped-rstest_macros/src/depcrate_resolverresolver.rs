// Generated macro for Resolver (trait)
macro_rules! Depcrate_resolverResolver {
() => {
// Module: crate::resolver
// Provides: {"Resolver"}
// Dependencies: {}
# [doc = " A trait that `resolve` the given ident to expression code to assign the value."] pub (crate) trait Resolver { fn resolve (& self , arg : & Pat) -> Option < Cow < '_ , Expr > > ; }
};
}
