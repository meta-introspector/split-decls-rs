// Generated macro for impl_742 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_742 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_742"}
// Dependencies: {}
impl UsesLifetimes for syn :: TypeParamBound { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: TypeParamBound :: Trait (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: TypeParamBound :: Lifetime (ref v) => v . uses_lifetimes (options , lifetimes) , _ => panic ! ("Unknown syn::TypeParamBound: {:?}" , self) , } } }
};
}
