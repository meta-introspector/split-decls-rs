// Generated macro for impl_16 (impl)
macro_rules! Depcrate_mapimpl_16 {
() => {
// Module: crate::map
// Provides: {"impl_16"}
// Dependencies: {}
impl < T , V > Iterator for ArenaMapIter < Idx < T > , V > { type Item = (Idx < T > , V) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { for next in self . iter . by_ref () { match Self :: mapper (next) { Some (r) => return Some (r) , None => continue , } } None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
