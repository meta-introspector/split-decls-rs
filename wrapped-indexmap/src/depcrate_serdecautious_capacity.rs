// Generated macro for cautious_capacity (function)
macro_rules! Depcrate_serdecautious_capacity {
() => {
// Module: crate::serde
// Provides: {"cautious_capacity"}
// Dependencies: {}
# [doc = " Limit our preallocated capacity from a deserializer `size_hint()`."] # [doc = ""] # [doc = " We do account for the `Bucket` overhead from its saved `hash` field, but we don't count the"] # [doc = " `RawTable` allocation or the fact that its raw capacity will be rounded up to a power of two."] # [doc = " The \"max\" is an arbitrary choice anyway, not something that needs precise adherence."] # [doc = ""] # [doc = " This is based on the internal `serde::de::size_hint::cautious(hint)` function."] pub (crate) fn cautious_capacity < K , V > (hint : Option < usize >) -> usize { const MAX_PREALLOC_BYTES : usize = 1024 * 1024 ; Ord :: min (hint . unwrap_or (0) , MAX_PREALLOC_BYTES / size_of :: < Bucket < K , V > > () ,) }
};
}
