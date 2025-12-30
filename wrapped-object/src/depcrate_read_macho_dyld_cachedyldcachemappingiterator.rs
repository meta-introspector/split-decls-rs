// Generated macro for DyldCacheMappingIterator (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheMappingIterator {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheMappingIterator"}
// Dependencies: {}
# [doc = " An iterator over all the mappings for one subcache in a dyld shared cache."] # [derive (Debug)] pub struct DyldCacheMappingIterator < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , iter : DyldCacheMappingVersionIterator < 'data , E > , }
};
}
