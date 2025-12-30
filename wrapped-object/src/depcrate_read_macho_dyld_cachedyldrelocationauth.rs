// Generated macro for DyldRelocationAuth (struct)
macro_rules! Depcrate_read_macho_dyld_cacheDyldRelocationAuth {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"DyldRelocationAuth"}
// Dependencies: {}
# [doc = " Pointer authentication data."] # [doc = ""] # [doc = " This is used for signing pointers for the arm64e ABI."] pub struct DyldRelocationAuth { # [doc = " The key used to generate the signed value."] pub key : macho :: PtrauthKey , # [doc = " The integer diversity value."] pub diversity : u16 , # [doc = " Whether the address should be blended with the diversity value."] pub addr_div : bool , }
};
}
