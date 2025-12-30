// Generated macro for Variants (struct)
macro_rules! Depcrate_subtags_variantsVariants {
() => {
// Module: crate::subtags::variants
// Provides: {"Variants"}
// Dependencies: {}
# [doc = " A list of variants (examples: `[\"macos\", \"posix\"]`, etc.)"] # [doc = ""] # [doc = " [`Variants`] stores a list of [`Variant`] subtags in a canonical form"] # [doc = " by sorting and deduplicating them."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::{variant, Variants};"] # [doc = ""] # [doc = " let mut v = vec![variant!(\"posix\"), variant!(\"macos\")];"] # [doc = " v.sort();"] # [doc = " v.dedup();"] # [doc = ""] # [doc = " let variants: Variants = Variants::from_vec_unchecked(v);"] # [doc = " assert_eq!(variants.to_string(), \"macos-posix\");"] # [doc = " ```"] # [derive (Default , Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord)] pub struct Variants (ShortBoxSlice < Variant >) ;
};
}
