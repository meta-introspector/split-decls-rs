// Generated macro for SearchBound (enum)
macro_rules! Depcrate_collections_btree_searchSearchBound {
() => {
// Module: crate::collections::btree::search
// Provides: {"SearchBound"}
// Dependencies: {}
pub (super) enum SearchBound < T > { # [doc = " An inclusive bound to look for, just like `Bound::Included(T)`."] Included (T) , # [doc = " An exclusive bound to look for, just like `Bound::Excluded(T)`."] Excluded (T) , # [doc = " An unconditional inclusive bound, just like `Bound::Unbounded`."] AllIncluded , # [doc = " An unconditional exclusive bound."] AllExcluded , }
};
}
