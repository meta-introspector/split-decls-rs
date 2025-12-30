// Generated macro for impl_689 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_689 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_689"}
// Dependencies: {}
impl < 'data , E , R > Iterator for DyldCacheMappingIterator < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = DyldCacheMapping < 'data , E , R > ; fn next (& mut self) -> Option < Self :: Item > { let info = match & mut self . iter { DyldCacheMappingVersionIterator :: V1 (iter) => DyldCacheMappingVersion :: V1 (iter . next () ?) , DyldCacheMappingVersionIterator :: V2 (iter) => DyldCacheMappingVersion :: V2 (iter . next () ?) , } ; Some (DyldCacheMapping { endian : self . endian , data : self . data , info , }) } }
};
}
