// Generated macro for SkipSet (struct)
macro_rules! Depcrate_setSkipSet {
() => {
// Module: crate::set
// Provides: {"SkipSet"}
// Dependencies: {}
# [doc = " A set based on a lock-free skip list."] # [doc = ""] # [doc = " This is an alternative to [`BTreeSet`] which supports"] # [doc = " concurrent access across multiple threads."] # [doc = ""] # [doc = " [`BTreeSet`]: std::collections::BTreeSet"] pub struct SkipSet < T > { inner : map :: SkipMap < T , () > , }
};
}
