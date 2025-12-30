// Generated macro for RefVisitor (struct)
macro_rules! Depcrate_lifetimesRefVisitor {
() => {
// Module: crate::lifetimes
// Provides: {"RefVisitor"}
// Dependencies: {}
struct RefVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , lts : Vec < Lifetime > , nested_elision_site_lts : Vec < Lifetime > , unelided_trait_object_lifetime : bool , }
};
}
