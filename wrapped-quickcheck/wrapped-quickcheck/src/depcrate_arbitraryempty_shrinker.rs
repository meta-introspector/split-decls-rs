// Generated macro for empty_shrinker (function)
macro_rules! Depcrate_arbitraryempty_shrinker {
() => {
// Module: crate::arbitrary
// Provides: {"empty_shrinker"}
// Dependencies: {}
# [doc = " Creates a shrinker with zero elements."] pub fn empty_shrinker < A : 'static > () -> Box < dyn Iterator < Item = A > > { Box :: new (empty ()) }
};
}
