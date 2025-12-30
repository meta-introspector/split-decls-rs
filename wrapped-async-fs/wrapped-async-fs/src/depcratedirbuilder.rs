// Generated macro for DirBuilder (struct)
macro_rules! DepcrateDirBuilder {
() => {
// Module: crate
// Provides: {"DirBuilder"}
// Dependencies: {}
# [doc = " A builder for creating directories with configurable options."] # [doc = ""] # [doc = " For Unix-specific options, import the [`DirBuilderExt`][`std::os::unix::fs::DirBuilderExt`]"] # [doc = " trait."] # [derive (Debug , Default)] pub struct DirBuilder { # [doc = " Set to `true` if non-existent parent directories should be created."] recursive : bool , # [doc = " Unix mode for newly created directories."] # [cfg (unix)] mode : Option < u32 > , }
};
}
