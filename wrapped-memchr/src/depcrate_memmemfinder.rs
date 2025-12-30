// Generated macro for Finder (struct)
macro_rules! Depcrate_memmemFinder {
() => {
// Module: crate::memmem
// Provides: {"Finder"}
// Dependencies: {}
# [doc = " A single substring searcher fixed to a particular needle."] # [doc = ""] # [doc = " The purpose of this type is to permit callers to construct a substring"] # [doc = " searcher that can be used to search haystacks without the overhead of"] # [doc = " constructing the searcher in the first place. This is a somewhat niche"] # [doc = " concern when it's necessary to re-use the same needle to search multiple"] # [doc = " different haystacks with as little overhead as possible. In general, using"] # [doc = " [`find`] is good enough, but `Finder` is useful when you can meaningfully"] # [doc = " observe searcher construction time in a profile."] # [doc = ""] # [doc = " When the `std` feature is enabled, then this type has an `into_owned`"] # [doc = " version which permits building a `Finder` that is not connected to"] # [doc = " the lifetime of its needle."] # [derive (Clone , Debug)] pub struct Finder < 'n > { needle : CowBytes < 'n > , searcher : Searcher , }
};
}
