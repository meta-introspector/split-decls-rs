// Generated macro for points_to (function)
macro_rules! Depcrate_matchers_points_to_matcherpoints_to {
() => {
// Module: crate::matchers::points_to_matcher
// Provides: {"points_to"}
// Dependencies: {}
# [doc = " Matches a reference pointing to a value matched by the [`Matcher`]"] # [doc = " `expected`."] # [doc = ""] # [doc = " This is useful for combining matchers, especially when working with"] # [doc = " iterators."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!(&123, points_to(eq(123)))?;"] # [doc = " verify_that!(vec![1,2,3], each(points_to(gt(0))))?;"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] pub fn points_to < MatcherT > (expected : MatcherT) -> PointsToMatcher < MatcherT > { PointsToMatcher { expected } }
};
}
