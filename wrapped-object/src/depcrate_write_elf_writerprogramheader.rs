// Generated macro for ProgramHeader (struct)
macro_rules! Depcrate_write_elf_writerProgramHeader {
() => {
// Module: crate::write::elf::writer
// Provides: {"ProgramHeader"}
// Dependencies: {}
# [doc = " Native endian version of [`elf::ProgramHeader64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct ProgramHeader { pub p_type : u32 , pub p_flags : u32 , pub p_offset : u64 , pub p_vaddr : u64 , pub p_paddr : u64 , pub p_filesz : u64 , pub p_memsz : u64 , pub p_align : u64 , }
};
}
