// Generated macro for impl_741 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_741 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_741"}
// Dependencies: {}
impl UsesLifetimes for syn :: GenericParam { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: GenericParam :: Lifetime (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericParam :: Type (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericParam :: Const (ref v) => v . uses_lifetimes (options , lifetimes) , } } }
};
}
