// Generated macro for impl_700 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_700 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_700"}
// Dependencies: {}
impl < T : UsesLifetimes > UsesLifetimes for Option < T > { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { self . as_ref () . map (| v | v . uses_lifetimes (options , lifetimes)) . unwrap_or_default () } }
};
}
