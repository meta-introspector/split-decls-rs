// Generated macro for impl_699 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_699 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_699"}
// Dependencies: {}
impl < T : UsesLifetimes , U > UsesLifetimes for Punctuated < T , U > { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { self . collect_lifetimes (options , lifetimes) } }
};
}
