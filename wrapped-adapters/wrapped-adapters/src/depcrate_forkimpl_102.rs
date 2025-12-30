// Generated macro for impl_102 (impl)
macro_rules! Depcrate_forkimpl_102 {
() => {
// Module: crate::fork
// Provides: {"impl_102"}
// Dependencies: {}
impl < P > MultiForkByMarkerProvider < P > { # [doc = " Create a provider that returns data from the first child provider supporting the marker."] # [doc = ""] # [doc = " See [`MultiForkByMarkerProvider`]."] pub fn new (providers : Vec < P >) -> Self { MultiForkByErrorProvider :: new_with_predicate (providers , MarkerNotFoundPredicate) } }
};
}
