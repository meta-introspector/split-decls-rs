// Generated macro for NT_VERSION (const)
macro_rules! Depcrate_elfNT_VERSION {
() => {
// Module: crate::elf
// Provides: {"NT_VERSION"}
// Dependencies: {}
# [doc = " Note type for version string."] # [doc = ""] # [doc = " This note may appear in object files."] # [doc = ""] # [doc = " It must be handled as a special case because it has no descriptor, and instead"] # [doc = " uses the note name as the version string."] pub const NT_VERSION : u32 = 1 ;
};
}
