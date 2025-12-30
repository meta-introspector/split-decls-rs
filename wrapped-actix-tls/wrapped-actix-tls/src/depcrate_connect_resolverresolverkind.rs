// Generated macro for ResolverKind (enum)
macro_rules! Depcrate_connect_resolverResolverKind {
() => {
// Module: crate::connect::resolver
// Provides: {"ResolverKind"}
// Dependencies: {}
# [derive (Clone)] enum ResolverKind { # [doc = " Built-in DNS resolver."] # [doc = ""] # [doc = " See [`std::net::ToSocketAddrs`] trait."] Default , # [doc = " Custom, user-provided DNS resolver."] Custom (Rc < dyn Resolve >) , }
};
}
