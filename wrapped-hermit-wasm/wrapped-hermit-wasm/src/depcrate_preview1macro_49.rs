// Generated macro for macro_49 (macro)
macro_rules! Depcrate_preview1macro_49 {
() => {
// Module: crate::preview1
// Provides: {"macro_49"}
// Dependencies: {}
bitflags ! { # [doc = " Options for opening files"] # [derive (Debug , Copy , Clone , Default)] pub (crate) struct Oflags : i32 { # [doc = " Create file if it does not exist."] const OFLAGS_CREAT = 1 << 0 ; # [doc = " Fail if not a directory."] const OFLAGS_DIRECTORY = 1 << 1 ; # [doc = " Fail if file already exists."] const OFLAGS_EXCL = 1 << 2 ; # [doc = " Truncate file to size 0."] const OFLAGS_TRUNC = 1 << 3 ; } }
};
}
