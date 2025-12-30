// Generated macro for impl_16 (impl)
macro_rules! Depcrate_matcherimpl_16 {
() => {
// Module: crate::matcher
// Provides: {"impl_16"}
// Dependencies: {}
impl RegexMatcher { # [doc = " Create a new matcher from the given pattern using the default"] # [doc = " configuration."] pub fn new (pattern : & str) -> Result < RegexMatcher , Error > { RegexMatcherBuilder :: new () . build (pattern) } }
};
}
