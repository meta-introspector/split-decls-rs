// Generated macro for impl_736 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_736 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_736"}
// Dependencies: {}
impl UsesLifetimes for syn :: TypePath { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { let mut hits = self . path . uses_lifetimes (options , lifetimes) ; if options . include_type_path_qself () { hits . extend (self . qself . uses_lifetimes (options , lifetimes)) ; } hits } }
};
}
