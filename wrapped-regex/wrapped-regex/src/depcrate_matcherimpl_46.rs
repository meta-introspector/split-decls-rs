// Generated macro for impl_46 (impl)
macro_rules! Depcrate_matcherimpl_46 {
() => {
// Module: crate::matcher
// Provides: {"impl_46"}
// Dependencies: {}
impl RegexMatcher { # [doc = " Create a new matcher from the given pattern using the default"] # [doc = " configuration."] pub fn new (pattern : & str) -> Result < RegexMatcher , Error > { RegexMatcherBuilder :: new () . build (pattern) } # [doc = " Create a new matcher from the given pattern using the default"] # [doc = " configuration, but matches lines terminated by `\\n`."] # [doc = ""] # [doc = " This is meant to be a convenience constructor for"] # [doc = " using a `RegexMatcherBuilder` and setting its"] # [doc = " [`line_terminator`](RegexMatcherBuilder::method.line_terminator) to"] # [doc = " `\\n`. The purpose of using this constructor is to permit special"] # [doc = " optimizations that help speed up line oriented search. These types of"] # [doc = " optimizations are only appropriate when matches span no more than one"] # [doc = " line. For this reason, this constructor will return an error if the"] # [doc = " given pattern contains a literal `\\n`. Other uses of `\\n` (such as in"] # [doc = " `\\s`) are removed transparently."] pub fn new_line_matcher (pattern : & str) -> Result < RegexMatcher , Error > { RegexMatcherBuilder :: new () . line_terminator (Some (b'\n')) . build (pattern) } }
};
}
