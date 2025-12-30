// Generated macro for SectionHeader (struct)
macro_rules! Depcrate_write_elf_writerSectionHeader {
() => {
// Module: crate::write::elf::writer
// Provides: {"SectionHeader"}
// Dependencies: {}
# [doc = " Native endian version of [`elf::SectionHeader64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct SectionHeader { pub name : Option < StringId > , pub sh_type : u32 , pub sh_flags : u64 , pub sh_addr : u64 , pub sh_offset : u64 , pub sh_size : u64 , pub sh_link : u32 , pub sh_info : u32 , pub sh_addralign : u64 , pub sh_entsize : u64 , }
};
}
