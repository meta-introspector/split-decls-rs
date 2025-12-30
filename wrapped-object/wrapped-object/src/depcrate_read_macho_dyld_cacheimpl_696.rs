// Generated macro for impl_696 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_696 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_696"}
// Dependencies: {}
impl < 'data , E , R > Iterator for DyldCacheRelocationIterator < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = Result < DyldRelocation > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . version { DyldCacheRelocationIteratorVersion :: None => Ok (None) , DyldCacheRelocationIteratorVersion :: V2 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V3 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V5 (iter) => iter . next () , } . transpose () } }
};
}
