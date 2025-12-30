// Generated macro for impl_100 (impl)
macro_rules! Depcrate_forkimpl_100 {
() => {
// Module: crate::fork
// Provides: {"impl_100"}
// Dependencies: {}
impl < P0 , P1 > ForkByMarkerProvider < P0 , P1 > { # [doc = " A provider that returns data from one of two child providers based on the marker."] # [doc = ""] # [doc = " See [`ForkByMarkerProvider`]."] pub fn new (p0 : P0 , p1 : P1) -> Self { ForkByErrorProvider :: new_with_predicate (p0 , p1 , MarkerNotFoundPredicate) } }
};
}
