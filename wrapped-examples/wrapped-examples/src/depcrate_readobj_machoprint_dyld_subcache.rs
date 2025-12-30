// Generated macro for print_dyld_subcache (function)
macro_rules! Depcrate_readobj_machoprint_dyld_subcache {
() => {
// Module: crate::readobj::macho
// Provides: {"print_dyld_subcache"}
// Dependencies: {}
pub (super) fn print_dyld_subcache (p : & mut Printer < '_ > , data : & [u8]) { if ! p . options . file { return ; } let Some (header) = DyldCacheHeader :: < Endianness > :: parse (data) . print_err (p) else { return ; } ; let Some ((_ , endian)) = header . parse_magic () . print_err (p) else { return ; } ; p . group ("DyldCacheHeader" , | p | { p . field_bytes ("Magic" , & header . magic) ; p . field_hex ("MappingOffset" , header . mapping_offset . get (endian)) ; p . field ("MappingCount" , header . mapping_count . get (endian)) ; p . field_hex ("ImagesOffset" , header . images_offset . get (endian)) ; p . field ("ImagesCount" , header . images_count . get (endian)) ; p . field_hex ("DyldBaseAddress" , header . dyld_base_address . get (endian)) ; }) ; if let Some (mappings) = header . mappings (endian , data) . print_err (p) { match mappings { DyldCacheMappingSlice :: V1 (info) => { for mapping in info . iter () { print_dyld_cache_mapping_info (p , endian , mapping) ; } } DyldCacheMappingSlice :: V2 (info) => { for mapping in info . iter () { print_dyld_cache_mapping_and_slide_info (p , endian , data , mapping) ; } } _ => panic ! ("If this case is hit, it means that someone added a variant to the (non-exhaustive) \
                 DyldCacheMappingSlice enum and forgot to update this example") , } } p . blank () ; }
};
}
