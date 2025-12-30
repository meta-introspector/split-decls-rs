// Generated macro for SectionHeader64 (struct)
macro_rules! Depcrate_elfSectionHeader64 {
() => {
// Module: crate::elf
// Provides: {"SectionHeader64"}
// Dependencies: {}
# [doc = " Section header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SectionHeader64 < E : Endian > { # [doc = " Section name."] # [doc = ""] # [doc = " This is an offset into the section header string table."] pub sh_name : U32 < E > , # [doc = " Section type. One of the `SHT_*` constants."] pub sh_type : U32 < E > , # [doc = " Section flags. A combination of the `SHF_*` constants."] pub sh_flags : U64 < E > , # [doc = " Section virtual address at execution."] pub sh_addr : U64 < E > , # [doc = " Section file offset."] pub sh_offset : U64 < E > , # [doc = " Section size in bytes."] pub sh_size : U64 < E > , # [doc = " Link to another section."] # [doc = ""] # [doc = " The section relationship depends on the `sh_type` value."] pub sh_link : U32 < E > , # [doc = " Additional section information."] # [doc = ""] # [doc = " The meaning of this field depends on the `sh_type` value."] pub sh_info : U32 < E > , # [doc = " Section alignment."] pub sh_addralign : U64 < E > , # [doc = " Entry size if the section holds a table."] pub sh_entsize : U64 < E > , }
};
}
