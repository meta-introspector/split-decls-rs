// Generated macro for DyldCacheMapping (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheMapping {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheMapping"}
// Dependencies: {}
# [doc = " Information about a mapping."] # [derive (Clone , Copy)] pub struct DyldCacheMapping < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , info : DyldCacheMappingVersion < 'data , E > , }
};
}
