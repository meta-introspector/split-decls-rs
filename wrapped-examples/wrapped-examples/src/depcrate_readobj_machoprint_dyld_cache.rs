// Generated macro for print_dyld_cache (function)
macro_rules! Depcrate_readobj_machoprint_dyld_cache {
() => {
// Module: crate::readobj::macho
// Provides: {"print_dyld_cache"}
// Dependencies: {}
pub (super) fn print_dyld_cache (p : & mut Printer < '_ > , data : & [u8] , subcache_data : & [& [u8]]) { print_dyld_subcache (p , data) ; for subcache in subcache_data { print_dyld_subcache (p , subcache) ; } if let Some (cache) = DyldCache :: < Endianness > :: parse (data , subcache_data) . print_err (p) { print_dyld_cache_images (p , & cache) ; } }
};
}
