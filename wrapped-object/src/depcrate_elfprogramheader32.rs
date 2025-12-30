// Generated macro for ProgramHeader32 (struct)
macro_rules! Depcrate_elfProgramHeader32 {
() => {
// Module: crate::elf
// Provides: {"ProgramHeader32"}
// Dependencies: {}
# [doc = " Program segment header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ProgramHeader32 < E : Endian > { # [doc = " Segment type. One of the `PT_*` constants."] pub p_type : U32 < E > , # [doc = " Segment file offset."] pub p_offset : U32 < E > , # [doc = " Segment virtual address."] pub p_vaddr : U32 < E > , # [doc = " Segment physical address."] pub p_paddr : U32 < E > , # [doc = " Segment size in the file."] pub p_filesz : U32 < E > , # [doc = " Segment size in memory."] pub p_memsz : U32 < E > , # [doc = " Segment flags. A combination of the `PF_*` constants."] pub p_flags : U32 < E > , # [doc = " Segment alignment."] pub p_align : U32 < E > , }
};
}
