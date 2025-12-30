// Generated macro for DyldSubCacheEntryV2 (struct)
macro_rules! Depcrate_machoDyldSubCacheEntryV2 {
() => {
// Module: crate::macho
// Provides: {"DyldSubCacheEntryV2"}
// Dependencies: {}
# [doc = " Added in dyld-1042.1, which shipped with macOS 13 / iOS 16."] # [doc = " Called `dyld_subcache_entry` as of dyld-1042.1."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldSubCacheEntryV2 < E : Endian > { # [doc = " The UUID of this subcache."] pub uuid : [u8 ; 16] , # [doc = " The offset of this subcache from the main cache base address."] pub cache_vm_offset : U64 < E > , # [doc = " The file name suffix of the subCache file, e.g. \".25.data\" or \".03.development\"."] pub file_suffix : [u8 ; 32] , }
};
}
