// Generated macro for impl_698 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_698 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_698"}
// Dependencies: {}
impl < T : UsesLifetimes > UsesLifetimes for Vec < T > { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { self . collect_lifetimes (options , lifetimes) } }
};
}
