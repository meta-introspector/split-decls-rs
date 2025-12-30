// Generated macro for SectionHeader32 (struct)
macro_rules! Depcrate_elfSectionHeader32 {
() => {
// Module: crate::elf
// Provides: {"SectionHeader32"}
// Dependencies: {}
# [doc = " Section header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SectionHeader32 < E : Endian > { # [doc = " Section name."] # [doc = ""] # [doc = " This is an offset into the section header string table."] pub sh_name : U32 < E > , # [doc = " Section type. One of the `SHT_*` constants."] pub sh_type : U32 < E > , # [doc = " Section flags. A combination of the `SHF_*` constants."] pub sh_flags : U32 < E > , # [doc = " Section virtual address at execution."] pub sh_addr : U32 < E > , # [doc = " Section file offset."] pub sh_offset : U32 < E > , # [doc = " Section size in bytes."] pub sh_size : U32 < E > , # [doc = " Link to another section."] # [doc = ""] # [doc = " The section relationship depends on the `sh_type` value."] pub sh_link : U32 < E > , # [doc = " Additional section information."] # [doc = ""] # [doc = " The meaning of this field depends on the `sh_type` value."] pub sh_info : U32 < E > , # [doc = " Section alignment."] pub sh_addralign : U32 < E > , # [doc = " Entry size if the section holds a table."] pub sh_entsize : U32 < E > , }
};
}
