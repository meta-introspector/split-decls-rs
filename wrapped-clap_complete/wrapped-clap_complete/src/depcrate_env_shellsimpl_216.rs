// Generated macro for impl_216 (impl)
macro_rules! Depcrate_env_shellsimpl_216 {
() => {
// Module: crate::env::shells
// Provides: {"impl_216"}
// Dependencies: {}
impl Zsh { # [doc = " Escape value string"] fn escape_value (string : & str) -> String { string . replace ('\\' , "\\\\") . replace (':' , "\\:") } # [doc = " Escape help string"] fn escape_help (string : & str) -> String { string . replace ('\\' , "\\\\") } }
};
}
