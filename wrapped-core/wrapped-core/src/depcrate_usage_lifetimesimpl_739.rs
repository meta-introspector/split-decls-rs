// Generated macro for impl_739 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_739 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_739"}
// Dependencies: {}
impl UsesLifetimes for syn :: WherePredicate { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: WherePredicate :: Type (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: WherePredicate :: Lifetime (ref v) => v . uses_lifetimes (options , lifetimes) , _ => panic ! ("Unknown syn::WherePredicate: {:?}" , self) , } } }
};
}
