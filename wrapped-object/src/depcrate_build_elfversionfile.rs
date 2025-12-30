// Generated macro for VersionFile (struct)
macro_rules! Depcrate_build_elfVersionFile {
() => {
// Module: crate::build::elf
// Provides: {"VersionFile"}
// Dependencies: {}
# [doc = " A filename used for GNU versioning."] # [doc = ""] # [doc = " Stored in [`VersionFiles`]."] # [derive (Debug)] pub struct VersionFile < 'data > { id : VersionFileId , # [doc = " Ignore this file when writing the ELF file."] pub delete : bool , # [doc = " The filename."] pub name : ByteString < 'data > , }
};
}
