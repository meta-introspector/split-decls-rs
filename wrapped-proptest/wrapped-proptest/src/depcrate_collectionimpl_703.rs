// Generated macro for impl_703 (impl)
macro_rules! Depcrate_collectionimpl_703 {
() => {
// Module: crate::collection
// Provides: {"impl_703"}
// Dependencies: {}
impl Default for SizeRange { # [doc = " Constructs a `SizeRange` equivalent to `size_range(0..PROPTEST_MAX_DEFAULT_SIZE_RANGE)`."] # [doc = " The max can be set with the `PROPTEST_MAX_DEFAULT_SIZE_RANGE` env var, which defaults to `100`."] fn default () -> Self { size_range (0 .. Config :: default () . max_default_size_range) } }
};
}
