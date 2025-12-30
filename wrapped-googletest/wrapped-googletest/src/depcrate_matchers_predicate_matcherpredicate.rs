// Generated macro for predicate (function)
macro_rules! Depcrate_matchers_predicate_matcherpredicate {
() => {
// Module: crate::matchers::predicate_matcher
// Provides: {"predicate"}
// Dependencies: {}
# [doc = " Creates a matcher based on the predicate provided."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!(3, predicate(|x: i32| x % 2 == 1))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " The predicate should take the subject type and return a"] # [doc = " boolean."] # [doc = ""] # [doc = " Note: even if the Rust compiler should be able to infer the type of"] # [doc = " the closure argument, it is likely that it won't."] # [doc = " See <https://github.com/rust-lang/rust/issues/12679> for update on this issue."] # [doc = " This is easily fixed by explicitly declaring the type of the argument"] pub fn predicate < P > (predicate : P) -> PredicateMatcher < P , NoDescription , NoDescription > { PredicateMatcher { predicate , positive_description : NoDescription , negative_description : NoDescription , } }
};
}
