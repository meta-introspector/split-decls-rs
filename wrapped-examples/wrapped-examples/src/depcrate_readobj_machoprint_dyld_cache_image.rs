// Generated macro for print_dyld_cache_image (function)
macro_rules! Depcrate_readobj_machoprint_dyld_cache_image {
() => {
// Module: crate::readobj::macho
// Provides: {"print_dyld_cache_image"}
// Dependencies: {}
fn print_dyld_cache_image (p : & mut Printer < '_ > , data : & [u8] , offset : u64 , cache : & DyldCache) { let Some (kind) = object :: FileKind :: parse_at (data , offset) . print_err (p) else { return ; } ; match kind { object :: FileKind :: MachO32 => macho :: print_macho32 (p , data , offset , Some (cache)) , object :: FileKind :: MachO64 => macho :: print_macho64 (p , data , offset , Some (cache)) , _ => writeln ! (p . w () , "Format: {:?}" , kind) . unwrap () , } }
};
}
