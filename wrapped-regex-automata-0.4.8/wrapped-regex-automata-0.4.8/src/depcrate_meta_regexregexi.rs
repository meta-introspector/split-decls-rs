// Generated macro for RegexI (struct)
macro_rules! Depcrate_meta_regexRegexI {
() => {
// Module: crate::meta::regex
// Provides: {"RegexI"}
// Dependencies: {}
# [doc = " The internal implementation of `Regex`, split out so that it can be wrapped"] # [doc = " in an `Arc`."] # [derive (Debug)] struct RegexI { # [doc = " The core matching engine."] # [doc = ""] # [doc = " Why is this reference counted when RegexI is already wrapped in an Arc?"] # [doc = " Well, we need to capture this in a closure to our `Pool` below in order"] # [doc = " to create new `Cache` values when needed. So since it needs to be in"] # [doc = " two places, we make it reference counted."] # [doc = ""] # [doc = " We make `RegexI` itself reference counted too so that `Regex` itself"] # [doc = " stays extremely small and very cheap to clone."] strat : Arc < dyn Strategy > , # [doc = " Metadata about the regexes driving the strategy. The metadata is also"] # [doc = " usually stored inside the strategy too, but we put it here as well"] # [doc = " so that we can get quick access to it (without virtual calls) before"] # [doc = " executing the regex engine. For example, we use this metadata to"] # [doc = " detect a subset of cases where we know a match is impossible, and can"] # [doc = " thus avoid calling into the strategy at all."] # [doc = ""] # [doc = " Since `RegexInfo` is stored in multiple places, it is also reference"] # [doc = " counted."] info : RegexInfo , }
};
}
