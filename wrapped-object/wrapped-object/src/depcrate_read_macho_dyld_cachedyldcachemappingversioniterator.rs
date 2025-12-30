// Generated macro for DyldCacheMappingVersionIterator (enum)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheMappingVersionIterator {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheMappingVersionIterator"}
// Dependencies: {}
# [derive (Debug)] enum DyldCacheMappingVersionIterator < 'data , E = Endianness > where E : Endian , { V1 (slice :: Iter < 'data , macho :: DyldCacheMappingInfo < E > >) , V2 (slice :: Iter < 'data , macho :: DyldCacheMappingAndSlideInfo < E > >) , }
};
}
