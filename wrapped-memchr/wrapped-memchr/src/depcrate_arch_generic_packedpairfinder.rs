// Generated macro for Finder (struct)
macro_rules! Depcrate_arch_generic_packedpairFinder {
() => {
// Module: crate::arch::generic::packedpair
// Provides: {"Finder"}
// Dependencies: {}
# [doc = " A generic architecture dependent \"packed pair\" finder."] # [doc = ""] # [doc = " This finder picks two bytes that it believes have high predictive power"] # [doc = " for indicating an overall match of a needle. Depending on whether"] # [doc = " `Finder::find` or `Finder::find_prefilter` is used, it reports offsets"] # [doc = " where the needle matches or could match. In the prefilter case, candidates"] # [doc = " are reported whenever the [`Pair`] of bytes given matches."] # [doc = ""] # [doc = " This is architecture dependent because it uses specific vector operations"] # [doc = " to look for occurrences of the pair of bytes."] # [doc = ""] # [doc = " This type is not meant to be exported and is instead meant to be used as"] # [doc = " the implementation for architecture specific facades. Why? Because it's a"] # [doc = " bit of a quirky API that requires `inline(always)` annotations. And pretty"] # [doc = " much everything has safety obligations due (at least) to the caller needing"] # [doc = " to inline calls into routines marked with"] # [doc = " `#[target_feature(enable = \"...\")]`."] # [derive (Clone , Copy , Debug)] pub (crate) struct Finder < V > { pair : Pair , v1 : V , v2 : V , min_haystack_len : usize , }
};
}
