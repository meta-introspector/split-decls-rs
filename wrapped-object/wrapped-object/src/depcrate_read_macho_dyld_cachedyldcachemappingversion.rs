// Generated macro for DyldCacheMappingVersion (enum)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheMappingVersion {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheMappingVersion"}
// Dependencies: {}
# [derive (Clone , Copy)] enum DyldCacheMappingVersion < 'data , E = Endianness > where E : Endian , { V1 (& 'data macho :: DyldCacheMappingInfo < E >) , V2 (& 'data macho :: DyldCacheMappingAndSlideInfo < E >) , }
};
}
