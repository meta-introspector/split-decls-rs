// Generated macro for macro_8060 (macro)
macro_rules! Depcrate_non_octal_unix_permissionsmacro_8060 {
() => {
// Module: crate::non_octal_unix_permissions
// Provides: {"macro_8060"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for non-octal values used to set Unix file permissions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " They will be converted into octal, creating potentially"] # [doc = " unintended file permissions."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use std::fs::OpenOptions;"] # [doc = " use std::os::unix::fs::OpenOptionsExt;"] # [doc = ""] # [doc = " let mut options = OpenOptions::new();"] # [doc = " options.mode(644);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " use std::fs::OpenOptions;"] # [doc = " use std::os::unix::fs::OpenOptionsExt;"] # [doc = ""] # [doc = " let mut options = OpenOptions::new();"] # [doc = " options.mode(0o644);"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub NON_OCTAL_UNIX_PERMISSIONS , correctness , "use of non-octal value to set unix file permissions, which will be translated into octal" }
};
}
