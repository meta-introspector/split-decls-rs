// Generated macro for DyldCacheRelocationIterator (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheRelocationIterator {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator over relocations in a mapping"] # [derive (Debug)] pub struct DyldCacheRelocationIterator < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { version : DyldCacheRelocationIteratorVersion < 'data , E , R > , }
};
}
