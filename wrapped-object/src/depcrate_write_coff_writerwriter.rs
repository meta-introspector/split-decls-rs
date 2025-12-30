// Generated macro for Writer (struct)
macro_rules! Depcrate_write_coff_writerWriter {
() => {
// Module: crate::write::coff::writer
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " A helper for writing COFF files."] # [doc = ""] # [doc = " Writing uses a two phase approach. The first phase builds up all of the information"] # [doc = " that may need to be known ahead of time:"] # [doc = " - build string table"] # [doc = " - reserve section indices"] # [doc = " - reserve symbol indices"] # [doc = " - reserve file ranges for headers and sections"] # [doc = ""] # [doc = " Some of the information has ordering requirements. For example, strings must be added"] # [doc = " to the string table before reserving the file range for the string table. There are debug"] # [doc = " asserts to check some of these requirements."] # [doc = ""] # [doc = " The second phase writes everything out in order. Thus the caller must ensure writing"] # [doc = " is in the same order that file ranges were reserved. There are debug asserts to assist"] # [doc = " with checking this."] # [allow (missing_debug_implementations)] pub struct Writer < 'a > { buffer : & 'a mut dyn WritableBuffer , len : usize , section_num : u16 , symtab_offset : u32 , symtab_num : u32 , strtab : StringTable < 'a > , strtab_len : usize , strtab_offset : u32 , strtab_data : Vec < u8 > , }
};
}
