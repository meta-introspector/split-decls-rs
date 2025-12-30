// Generated macro for impl_3435 (impl)
macro_rules! Depcrate_lifetimesimpl_3435 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3435"}
// Dependencies: {}
impl < 'a , 'tcx > RefVisitor < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx >) -> Self { Self { cx , lts : Vec :: new () , nested_elision_site_lts : Vec :: new () , unelided_trait_object_lifetime : false , } } fn all_lts (& self) -> Vec < Lifetime > { self . lts . iter () . chain (self . nested_elision_site_lts . iter ()) . copied () . collect :: < Vec < _ > > () } fn abort (& self) -> bool { self . unelided_trait_object_lifetime } }
};
}
