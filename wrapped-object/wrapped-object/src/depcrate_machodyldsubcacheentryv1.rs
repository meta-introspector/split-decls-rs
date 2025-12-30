// Generated macro for DyldSubCacheEntryV1 (struct)
macro_rules! Depcrate_machoDyldSubCacheEntryV1 {
() => {
// Module: crate::macho
// Provides: {"DyldSubCacheEntryV1"}
// Dependencies: {}
# [doc = " Added in dyld-940, which shipped with macOS 12 / iOS 15."] # [doc = " Originally called `dyld_subcache_entry`, renamed to `dyld_subcache_entry_v1`"] # [doc = " in dyld-1042.1."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldSubCacheEntryV1 < E : Endian > { # [doc = " The UUID of this subcache."] pub uuid : [u8 ; 16] , # [doc = " The offset of this subcache from the main cache base address."] pub cache_vm_offset : U64 < E > , }
};
}
