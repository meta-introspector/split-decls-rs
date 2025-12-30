// Generated macro for big_archive (module)
macro_rules! Depcrate_archivebig_archive {
() => {
// Module: crate::archive
// Provides: {"big_archive"}
// Dependencies: {}
pub (crate) mod big_archive { # [repr (C)] pub (crate) struct BigArMemHdrType { # [doc = " File member size in decimal"] size : [u8 ; 20] , # [doc = " Next member offset in decimal"] next_offset : [u8 ; 20] , # [doc = " Previous member offset in decimal"] prev_offset : [u8 ; 20] , last_modified : [u8 ; 12] , uid : [u8 ; 12] , gid : [u8 ; 12] , access_mode : [u8 ; 12] , # [doc = " File member name length in decimal"] name_len : [u8 ; 4] , terminator : [u8 ; 2] , } # [doc = " Fixed-Length Header."] # [repr (C)] pub (crate) struct FixLenHdr { # [doc = " Big archive magic string."] magic : [u8 ; 8] , # [doc = " Offset to member table."] mem_offset : [u8 ; 20] , # [doc = " Offset to global symbol table."] glob_sym_offset : [u8 ; 20] , # [doc = " Offset global symbol table for 64-bit objects."] glob_sym64_offset : [u8 ; 20] , # [doc = " Offset to first archive member."] first_child_offset : [u8 ; 20] , # [doc = " Offset to last archive member."] last_child_offset : [u8 ; 20] , # [doc = " Offset to first mem on free list."] free_offset : [u8 ; 20] , } }
};
}
