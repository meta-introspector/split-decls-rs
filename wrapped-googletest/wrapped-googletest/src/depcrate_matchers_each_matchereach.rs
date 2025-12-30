// Generated macro for each (function)
macro_rules! Depcrate_matchers_each_matchereach {
() => {
// Module: crate::matchers::each_matcher
// Provides: {"each"}
// Dependencies: {}
# [doc = " Matches a container all of whose elements are matched by the matcher"] # [doc = " `inner`."] # [doc = ""] # [doc = " `T` must implement [`IntoIterator`]. This"] # [doc = " includes `&Vec`, arrays, and slices."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # use std::collections::HashSet;"] # [doc = " # fn should_pass_1() -> Result<()> {"] # [doc = " let value = vec![1, 2, 3];"] # [doc = " verify_that!(value, each(gt(&0)))?;  // Passes"] # [doc = " let array_value = [1, 2, 3];"] # [doc = " verify_that!(array_value, each(gt(0)))?;  // Passes"] # [doc = " let slice_value = &[1, 2, 3];"] # [doc = " verify_that!(slice_value, each(gt(&0)))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail() -> Result<()> {"] # [doc = " #     let value = vec![1, 2, 3];"] # [doc = " verify_that!(value, each(lt(&2)))?;  // Fails: 2 and 3 are not less than 2"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = ""] # [doc = " # fn should_pass_2() -> Result<()> {"] # [doc = " let value: HashSet<i32> = [1, 2, 3].into();"] # [doc = " verify_that!(value, each(gt(&0)))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass_1().unwrap();"] # [doc = " # should_fail().unwrap_err();"] # [doc = " # should_pass_2().unwrap();"] # [doc = " ```"] pub fn each < MatcherT > (inner : MatcherT) -> EachMatcher < MatcherT > { EachMatcher { inner } }
};
}
