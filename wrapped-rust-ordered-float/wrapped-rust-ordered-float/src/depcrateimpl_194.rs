// Generated macro for impl_194 (impl)
macro_rules! Depcrateimpl_194 {
() => {
// Module: crate
// Provides: {"impl_194"}
// Dependencies: {}
impl < T : FloatCore > Bounded for NotNan < T > { # [inline] fn min_value () -> Self { NotNan (T :: min_value ()) } # [inline] fn max_value () -> Self { NotNan (T :: max_value ()) } }
};
}
