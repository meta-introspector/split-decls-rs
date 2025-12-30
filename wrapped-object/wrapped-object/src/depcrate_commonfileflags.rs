// Generated macro for FileFlags (enum)
macro_rules! Depcrate_commonFileFlags {
() => {
// Module: crate::common
// Provides: {"FileFlags"}
// Dependencies: {}
# [doc = " File flags that are specific to each file format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum FileFlags { # [doc = " No file flags."] None , # [doc = " ELF file flags."] Elf { # [doc = " `os_abi` field in the ELF file header."] os_abi : u8 , # [doc = " `abi_version` field in the ELF file header."] abi_version : u8 , # [doc = " `e_flags` field in the ELF file header."] e_flags : u32 , } , # [doc = " Mach-O file flags."] MachO { # [doc = " `flags` field in the Mach-O file header."] flags : u32 , } , # [doc = " COFF file flags."] Coff { # [doc = " `Characteristics` field in the COFF file header."] characteristics : u16 , } , # [doc = " XCOFF file flags."] Xcoff { # [doc = " `f_flags` field in the XCOFF file header."] f_flags : u16 , } , }
};
}
