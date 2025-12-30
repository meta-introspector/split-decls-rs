// Generated macro for impl_17 (impl)
macro_rules! Depcrate_mapimpl_17 {
() => {
// Module: crate::map
// Provides: {"impl_17"}
// Dependencies: {}
impl < T , V > DoubleEndedIterator for ArenaMapIter < Idx < T > , V > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { while let Some (next_back) = self . iter . next_back () { match Self :: mapper (next_back) { Some (r) => return Some (r) , None => continue , } } None } }
};
}
