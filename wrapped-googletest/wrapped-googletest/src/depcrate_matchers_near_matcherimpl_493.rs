// Generated macro for impl_493 (impl)
macro_rules! Depcrate_matchers_near_matcherimpl_493 {
() => {
// Module: crate::matchers::near_matcher
// Provides: {"impl_493"}
// Dependencies: {}
impl < T : Debug > NearMatcher < T > { # [doc = " Configures this instance to treat two NaNs as equal."] # [doc = ""] # [doc = " This behaviour differs from the IEEE standad for floating point which"] # [doc = " treats two NaNs as infinitely far apart."] pub fn nans_are_equal (mut self) -> Self { self . nans_are_equal = true ; self } # [doc = " Configures this instance to treat two NaNs as not equal."] # [doc = ""] # [doc = " This behaviour complies with the IEEE standad for floating point. It is"] # [doc = " the default behaviour for this matcher, so invoking this method is"] # [doc = " usually redunant."] pub fn nans_are_not_equal (mut self) -> Self { self . nans_are_equal = false ; self } }
};
}
