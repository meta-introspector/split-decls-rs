// Generated macro for impl_535 (impl)
macro_rules! Depcrate_matchers_predicate_matcherimpl_535 {
() => {
// Module: crate::matchers::predicate_matcher
// Provides: {"impl_535"}
// Dependencies: {}
impl < P > PredicateMatcher < P , NoDescription , NoDescription > { # [doc = " Configures this instance to provide a more meaningful description."] # [doc = ""] # [doc = " For example, to make sure the error message is more useful"] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::matchers::{predicate, PredicateMatcher};"] # [doc = " # let _ ="] # [doc = " predicate(|x: i32| x % 2 == 1)"] # [doc = "     .with_description(\"is odd\", \"is even\")"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " This is optional as it only provides value when the test fails."] # [doc = ""] # [doc = " Description can be passed by `&str`, `String` or `Fn() -> Into<String>`."] pub fn with_description < D1 : PredicateDescription , D2 : PredicateDescription > (self , positive_description : D1 , negative_description : D2 ,) -> PredicateMatcher < P , D1 , D2 > { PredicateMatcher { predicate : self . predicate , positive_description , negative_description } } }
};
}
