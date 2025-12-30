// Generated macro for macro_8951 (macro)
macro_rules! Depcrate_permissions_set_readonly_falsemacro_8951 {
() => {
// Module: crate::permissions_set_readonly_false
// Provides: {"macro_8951"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `std::fs::Permissions.set_readonly` with argument `false`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " On Unix platforms this results in the file being world writable,"] # [doc = " equivalent to `chmod a+w <file>`."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::fs::File;"] # [doc = " let f = File::create(\"foo.txt\").unwrap();"] # [doc = " let metadata = f.metadata().unwrap();"] # [doc = " let mut permissions = metadata.permissions();"] # [doc = " permissions.set_readonly(false);"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub PERMISSIONS_SET_READONLY_FALSE , suspicious , "Checks for calls to `std::fs::Permissions.set_readonly` with argument `false`" }
};
}
