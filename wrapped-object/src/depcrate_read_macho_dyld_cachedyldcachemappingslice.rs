// Generated macro for DyldCacheMappingSlice (enum)
macro_rules! Depcrate_read_macho_dyld_cacheDyldCacheMappingSlice {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldCacheMappingSlice"}
// Dependencies: {}
# [doc = " The array of mappings for a single dyld cache file."] # [doc = ""] # [doc = " The mappings gained slide info in dyld-832.7 (macOS 11)"] # [doc = " so this is an enum of the two possible slice types."] # [derive (Debug , Clone , Copy)] # [non_exhaustive] pub enum DyldCacheMappingSlice < 'data , E : Endian = Endianness > { # [doc = " V1, used before dyld-832.7."] V1 (& 'data [macho :: DyldCacheMappingInfo < E >]) , # [doc = " V2, used since dyld-832.7."] V2 (& 'data [macho :: DyldCacheMappingAndSlideInfo < E >]) , }
};
}
