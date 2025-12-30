// Generated macro for RegexCaptures (struct)
macro_rules! Depcrate_matcherRegexCaptures {
() => {
// Module: crate::matcher
// Provides: {"RegexCaptures"}
// Dependencies: {}
# [doc = " Represents the match offsets of each capturing group in a match."] # [doc = ""] # [doc = " The first, or `0`th capture group, always corresponds to the entire match"] # [doc = " and is guaranteed to be present when a match occurs. The next capture"] # [doc = " group, at index `1`, corresponds to the first capturing group in the regex,"] # [doc = " ordered by the position at which the left opening parenthesis occurs."] # [doc = ""] # [doc = " Note that not all capturing groups are guaranteed to be present in a match."] # [doc = " For example, in the regex, `(?P<foo>\\w)|(?P<bar>\\W)`, only one of `foo`"] # [doc = " or `bar` will ever be set in any given match."] # [doc = ""] # [doc = " In order to access a capture group by name, you'll need to first find the"] # [doc = " index of the group using the corresponding matcher's `capture_index`"] # [doc = " method, and then use that index with `RegexCaptures::get`."] # [derive (Clone , Debug)] pub struct RegexCaptures { # [doc = " Where the locations are stored."] locs : CaptureLocations , }
};
}
