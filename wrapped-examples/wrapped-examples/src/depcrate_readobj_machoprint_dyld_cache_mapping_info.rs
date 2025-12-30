// Generated macro for print_dyld_cache_mapping_info (function)
macro_rules! Depcrate_readobj_machoprint_dyld_cache_mapping_info {
() => {
// Module: crate::readobj::macho
// Provides: {"print_dyld_cache_mapping_info"}
// Dependencies: {}
pub (super) fn print_dyld_cache_mapping_info (p : & mut Printer < '_ > , endian : Endianness , mapping : & DyldCacheMappingInfo < Endianness > ,) { p . group ("DyldCacheMappingInfo" , | p | { p . field_hex ("Address" , mapping . address . get (endian)) ; p . field_hex ("Size" , mapping . size . get (endian)) ; p . field_hex ("FileOffset" , mapping . file_offset . get (endian)) ; p . field_hex ("MaxProt" , mapping . max_prot . get (endian)) ; p . flags (mapping . max_prot . get (endian) , 0 , FLAGS_VM) ; p . field_hex ("InitProt" , mapping . init_prot . get (endian)) ; p . flags (mapping . init_prot . get (endian) , 0 , FLAGS_VM) ; }) ; }
};
}
