// Generated macro for LocaleFallbackIteratorInner (struct)
macro_rules! Depcrate_fallbackLocaleFallbackIteratorInner {
() => {
// Module: crate::fallback
// Provides: {"LocaleFallbackIteratorInner"}
// Dependencies: {}
# [doc = " Inner iteration type. Does not own the item under fallback."] # [derive (Debug)] struct LocaleFallbackIteratorInner < 'a > { likely_subtags : & 'a LikelySubtagsForLanguage < 'a > , parents : & 'a Parents < 'a > , config : LocaleFallbackConfig , backup_subdivision : Option < Subtag > , backup_variant : Option < Variant > , backup_region : Option < Region > , max_script : Option < Script > , }
};
}
