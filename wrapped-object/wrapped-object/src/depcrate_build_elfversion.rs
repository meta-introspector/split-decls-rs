// Generated macro for Version (struct)
macro_rules! Depcrate_build_elfVersion {
() => {
// Module: crate::build::elf
// Provides: {"Version"}
// Dependencies: {}
# [doc = " A version for a symbol."] # [derive (Debug)] pub struct Version < 'data > { id : VersionId , # [doc = " The data for this version."] pub data : VersionData < 'data > , # [doc = " Ignore this version when writing the ELF file."] pub delete : bool , }
};
}
