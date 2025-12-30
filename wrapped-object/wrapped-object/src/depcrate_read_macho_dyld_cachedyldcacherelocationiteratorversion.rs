// Generated macro for DyldCacheRelocationIteratorVersion (enum)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheRelocationIteratorVersion {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheRelocationIteratorVersion"}
// Dependencies: {}
# [derive (Debug)] enum DyldCacheRelocationIteratorVersion < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { None , V2 (DyldCacheRelocationIteratorV2 < 'data , E , R >) , V3 (DyldCacheRelocationIteratorV3 < 'data , E , R >) , V5 (DyldCacheRelocationIteratorV5 < 'data , E , R >) , }
};
}
