// Generated macro for maybe_replace_first (function)
macro_rules! Depcrate_provider_pattern_runtime_helpersmaybe_replace_first {
() => {
// Module: crate::provider::pattern::runtime::helpers
// Provides: {"maybe_replace_first"}
// Dependencies: {}
# [doc = " Helper function which takes a runtime `Pattern` and calls"] # [doc = " the callback function passing each item until the callback"] # [doc = " returns a new value."] # [doc = ""] # [doc = " When that happens, the `Pattern` is turned into an owned one,"] # [doc = " and the old item gets replaced with the new one."] # [doc = ""] # [doc = " The utility of this function is to allow for a single"] # [doc = " item to be replaced allocating the `Pattern` only if needed."] # [doc = ""] # [doc = " For a variant that replaces all matching instances, see `maybe_replace`."] # [cfg (feature = "datagen")] pub fn maybe_replace_first (pattern : & mut Pattern , f : impl Fn (& PatternItem) -> Option < PatternItem >) { let result = pattern . items . iter () . enumerate () . find_map (| (i , item) | f (& item) . map (| result | (i , result))) ; # [expect (clippy :: indexing_slicing)] if let Some ((i , result)) = result { pattern . items . to_mut_slice () [i] = result . to_unaligned () ; } }
};
}
