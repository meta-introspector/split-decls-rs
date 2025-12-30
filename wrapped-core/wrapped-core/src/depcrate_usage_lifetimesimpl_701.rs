// Generated macro for impl_701 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_701 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_701"}
// Dependencies: {}
impl UsesLifetimes for Lifetime { fn uses_lifetimes < 'a > (& self , _ : & Options , lifetimes : & 'a LifetimeSet) -> LifetimeRefSet < 'a > { lifetimes . iter () . filter (| lt | * lt == self) . collect () } }
};
}
