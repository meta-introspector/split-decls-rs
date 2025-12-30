// Generated macro for rand_unicode_with_range (function)
macro_rules! Depcrate_testsrand_unicode_with_range {
() => {
// Module: crate::tests
// Provides: {"rand_unicode_with_range"}
// Dependencies: {}
# [doc = " [`proptest::strategy::Strategy`] that generates [`String`]s with up to `len` bytes"] pub (crate) fn rand_unicode_with_range (range : impl Into < SizeRange > ,) -> impl Strategy < Value = String > { proptest :: collection :: vec (proptest :: char :: any () , range) . prop_map (| v | v . into_iter () . collect ()) }
};
}
