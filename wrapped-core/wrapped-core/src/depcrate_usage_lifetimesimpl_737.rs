// Generated macro for impl_737 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_737 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_737"}
// Dependencies: {}
impl UsesLifetimes for syn :: ReturnType { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { if let syn :: ReturnType :: Type (_ , ref ty) = * self { ty . uses_lifetimes (options , lifetimes) } else { Default :: default () } } }
};
}
