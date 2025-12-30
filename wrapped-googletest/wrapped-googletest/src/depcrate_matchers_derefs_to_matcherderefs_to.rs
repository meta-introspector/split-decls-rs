// Generated macro for derefs_to (function)
macro_rules! Depcrate_matchers_derefs_to_matcherderefs_to {
() => {
// Module: crate::matchers::derefs_to_matcher
// Provides: {"derefs_to"}
// Dependencies: {}
# [doc = " Dereferences the `actual` value and verifies that the returned reference"] # [doc = " matches the `inner` matcher."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::{matchers::{derefs_to, eq}, verify_that};"] # [doc = " verify_that!(Box::new(123), derefs_to(eq(&123)))"] # [doc = " #    .unwrap()"] # [doc = " ```"] pub fn derefs_to < Inner > (inner : Inner) -> DerefsTo < Inner > { DerefsTo { inner } }
};
}
