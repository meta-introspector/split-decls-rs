// Generated macro for impl_682 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_682 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_682"}
// Dependencies: {}
impl < 'data , 'cache , E , R > Iterator for DyldCacheImageIterator < 'data , 'cache , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = DyldCacheImage < 'data , 'cache , E , R > ; fn next (& mut self) -> Option < DyldCacheImage < 'data , 'cache , E , R > > { let image_info = self . iter . next () ? ; Some (DyldCacheImage { cache : self . cache , image_info , }) } }
};
}
