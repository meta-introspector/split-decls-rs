// Generated macro for single_shrinker (function)
macro_rules! Depcrate_arbitrarysingle_shrinker {
() => {
// Module: crate::arbitrary
// Provides: {"single_shrinker"}
// Dependencies: {}
# [doc = " Creates a shrinker with a single element."] pub fn single_shrinker < A : 'static > (value : A) -> Box < dyn Iterator < Item = A > > { Box :: new (once (value)) }
};
}
