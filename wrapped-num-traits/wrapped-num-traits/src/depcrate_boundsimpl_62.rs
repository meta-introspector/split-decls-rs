// Generated macro for impl_62 (impl)
macro_rules! Depcrate_boundsimpl_62 {
() => {
// Module: crate::bounds
// Provides: {"impl_62"}
// Dependencies: {}
impl < T : Bounded > Bounded for Wrapping < T > { fn min_value () -> Self { Wrapping (T :: min_value ()) } fn max_value () -> Self { Wrapping (T :: max_value ()) } }
};
}
