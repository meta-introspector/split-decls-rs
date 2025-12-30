// Generated macro for is_empty (function)
macro_rules! Depcrate_matchers_empty_matcheris_empty {
() => {
// Module: crate::matchers::empty_matcher
// Provides: {"is_empty"}
// Dependencies: {}
# [doc = " Matches an empty container."] # [doc = ""] # [doc = " `T` can be any container that implements `IntoIterator`. For instance, `T`"] # [doc = " can be the reference of a common container like `&Vec` and"] # [doc = " [`&HashSet`][std::collections::HashSet]."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # use std::collections::HashSet;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " let value: Vec<i32> = vec![];"] # [doc = " verify_that!(value, is_empty())?;"] # [doc = " let value: HashSet<i32> = HashSet::new();"] # [doc = " verify_that!(value, is_empty())?;"] # [doc = " let value: &[u32] = &[];"] # [doc = " verify_that!(value, is_empty())?;"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] pub fn is_empty () -> EmptyMatcher { EmptyMatcher }
};
}
