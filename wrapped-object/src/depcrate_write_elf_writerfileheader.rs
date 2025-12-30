// Generated macro for FileHeader (struct)
macro_rules! Depcrate_write_elf_writerFileHeader {
() => {
// Module: crate::write::elf::writer
// Provides: {"FileHeader"}
// Dependencies: {}
# [doc = " Native endian version of [`elf::FileHeader64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct FileHeader { pub os_abi : u8 , pub abi_version : u8 , pub e_type : u16 , pub e_machine : u16 , pub e_entry : u64 , pub e_flags : u32 , }
};
}
