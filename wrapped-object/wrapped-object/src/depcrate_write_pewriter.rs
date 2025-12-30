// Generated macro for Writer (struct)
macro_rules! Depcrate_write_peWriter {
() => {
// Module: crate::write::pe
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " A helper for writing PE files."] # [doc = ""] # [doc = " Writing uses a two phase approach. The first phase reserves file ranges and virtual"] # [doc = " address ranges for everything in the order that they will be written."] # [doc = ""] # [doc = " The second phase writes everything out in order. Thus the caller must ensure writing"] # [doc = " is in the same order that file ranges were reserved."] # [allow (missing_debug_implementations)] pub struct Writer < 'a > { is_64 : bool , section_alignment : u32 , file_alignment : u32 , buffer : & 'a mut dyn WritableBuffer , len : u32 , virtual_len : u32 , headers_len : u32 , code_address : u32 , data_address : u32 , code_len : u32 , data_len : u32 , bss_len : u32 , nt_headers_offset : u32 , data_directories : Vec < DataDirectory > , section_header_num : u16 , sections : Vec < Section > , symbol_offset : u32 , symbol_num : u32 , reloc_blocks : Vec < RelocBlock > , relocs : Vec < U16 < LE > > , reloc_offset : u32 , }
};
}
