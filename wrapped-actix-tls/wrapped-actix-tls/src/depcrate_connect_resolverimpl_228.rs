// Generated macro for impl_228 (impl)
macro_rules! Depcrate_connect_resolverimpl_228 {
() => {
// Module: crate::connect::resolver
// Provides: {"impl_228"}
// Dependencies: {}
impl Resolver { # [doc = " Constructs a new resolver factory with a custom resolver."] pub fn custom (resolver : impl Resolve + 'static) -> Self { Self { resolver : ResolverService :: custom (resolver) , } } # [doc = " Returns a new resolver service."] pub fn service (& self) -> ResolverService { self . resolver . clone () } }
};
}
