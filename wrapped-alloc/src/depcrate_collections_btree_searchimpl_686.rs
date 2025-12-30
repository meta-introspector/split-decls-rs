// Generated macro for impl_686 (impl)
macro_rules! Depcrate_collections_btree_searchimpl_686 {
() => {
// Module: crate::collections::btree::search
// Provides: {"impl_686"}
// Dependencies: {}
impl < T > SearchBound < T > { pub (super) fn from_range (range_bound : Bound < T >) -> Self { match range_bound { Bound :: Included (t) => Included (t) , Bound :: Excluded (t) => Excluded (t) , Bound :: Unbounded => AllIncluded , } } }
};
}
