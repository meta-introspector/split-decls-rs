// Generated macro for macro_1289 (macro)
macro_rules! Depcrate_create_dirmacro_1289 {
() => {
// Module: crate::create_dir
// Provides: {"macro_1289"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks usage of `std::fs::create_dir` and suggest using `std::fs::create_dir_all` instead."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Sometimes `std::fs::create_dir` is mistakenly chosen over `std::fs::create_dir_all`,"] # [doc = " resulting in failure when more than one directory needs to be created or when the directory already exists."] # [doc = " Crates which never need to specifically create a single directory may wish to prevent this mistake."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " std::fs::create_dir(\"foo\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " std::fs::create_dir_all(\"foo\");"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub CREATE_DIR , restriction , "calling `std::fs::create_dir` instead of `std::fs::create_dir_all`" }
};
}
