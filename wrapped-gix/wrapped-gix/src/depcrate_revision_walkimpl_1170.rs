// Generated macro for impl_1170 (impl)
macro_rules! Depcrate_revision_walkimpl_1170 {
() => {
// Module: crate::revision::walk
// Provides: {"impl_1170"}
// Dependencies: {}
impl Sorting { fn into_simple (self) -> Option < gix_traverse :: commit :: simple :: Sorting > { Some (match self { Sorting :: BreadthFirst => gix_traverse :: commit :: simple :: Sorting :: BreadthFirst , Sorting :: ByCommitTime (order) => gix_traverse :: commit :: simple :: Sorting :: ByCommitTime (order) , Sorting :: ByCommitTimeCutoff { seconds , order } => { gix_traverse :: commit :: simple :: Sorting :: ByCommitTimeCutoff { order , seconds } } }) } }
};
}
