// Generated macro for macro_9076 (macro)
macro_rules! Depcrate_pub_underscore_fieldsmacro_9076 {
() => {
// Module: crate::pub_underscore_fields
// Provides: {"macro_9076"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks whether any field of the struct is prefixed with an `_` (underscore) and also marked"] # [doc = " `pub` (public)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Fields prefixed with an `_` are inferred as unused, which suggests it should not be marked"] # [doc = " as `pub`, because marking it as `pub` infers it will be used."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " struct FileHandle {"] # [doc = "     pub _descriptor: usize,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " struct FileHandle {"] # [doc = "     _descriptor: usize,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " OR"] # [doc = ""] # [doc = " ```rust"] # [doc = " struct FileHandle {"] # [doc = "     pub descriptor: usize,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub PUB_UNDERSCORE_FIELDS , pedantic , "struct field prefixed with underscore and marked public" }
};
}
