// Generated macro for SkipMap (struct)
macro_rules! Depcrate_mapSkipMap {
() => {
// Module: crate::map
// Provides: {"SkipMap"}
// Dependencies: {}
# [doc = " An ordered map based on a lock-free skip list."] # [doc = ""] # [doc = " This is an alternative to [`BTreeMap`] which supports"] # [doc = " concurrent access across multiple threads."] # [doc = ""] # [doc = " [`BTreeMap`]: std::collections::BTreeMap"] pub struct SkipMap < K , V > { inner : base :: SkipList < K , V > , }
};
}
