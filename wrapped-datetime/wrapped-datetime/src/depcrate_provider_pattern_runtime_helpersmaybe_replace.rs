// Generated macro for maybe_replace (function)
macro_rules! Depcrate_provider_pattern_runtime_helpersmaybe_replace {
() => {
// Module: crate::provider::pattern::runtime::helpers
// Provides: {"maybe_replace"}
// Dependencies: {}
# [doc = " Helper function which takes a runtime `Pattern` and calls"] # [doc = " the callback function passing each item."] # [doc = " If the callback returns a new value, the old one gets"] # [doc = " replaced with it."] # [doc = ""] # [doc = " The utility of this function is to allow for a pattern"] # [doc = " to be altered, allocating the `Pattern` only if needed."] # [doc = ""] # [doc = " For a variant that replaces just the first matching instance,"] # [doc = " see `maybe_replace_first`."] # [cfg (feature = "datagen")] pub fn maybe_replace (pattern : & mut Pattern , f : impl Fn (& PatternItem) -> Option < PatternItem >) { let result = pattern . items . iter () . enumerate () . find_map (| (i , item) | f (& item) . map (| result | (i , result))) ; # [expect (clippy :: indexing_slicing)] if let Some ((i , result)) = result { let owned = pattern . items . to_mut_slice () ; owned [i] = result . to_unaligned () ; owned . iter_mut () . skip (i) . for_each (| item | { if let Some (new_item) = f (& PatternItem :: from_unaligned (* item)) { * item = new_item . to_unaligned () ; } }) ; } }
};
}
