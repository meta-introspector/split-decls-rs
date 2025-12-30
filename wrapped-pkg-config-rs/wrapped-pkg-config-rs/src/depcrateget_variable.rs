// Generated macro for get_variable (function)
macro_rules! Depcrateget_variable {
() => {
// Module: crate
// Provides: {"get_variable"}
// Dependencies: {}
# [doc = " Run `pkg-config` to get the value of a variable from a package using"] # [doc = " `--variable`."] # [doc = ""] # [doc = " The content of `PKG_CONFIG_SYSROOT_DIR` is not injected in paths that are"] # [doc = " returned by `pkg-config --variable`, which makes them unsuitable to use"] # [doc = " during cross-compilation unless specifically designed to be used"] # [doc = " at that time."] pub fn get_variable (package : & str , variable : & str) -> Result < String , Error > { let arg = format ! ("--variable={}" , variable) ; let cfg = Config :: new () ; let out = cfg . run (package , & [& arg]) ? ; Ok (str :: from_utf8 (& out) . unwrap () . trim_end () . to_owned ()) }
};
}
