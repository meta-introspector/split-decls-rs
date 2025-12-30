// Generated macro for DyldFile (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldFile {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldFile"}
// Dependencies: {}
# [doc = " The data for one file in the cache."] # [derive (Debug)] struct DyldFile < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { data : R , mappings : DyldCacheMappingSlice < 'data , E > , }
};
}
