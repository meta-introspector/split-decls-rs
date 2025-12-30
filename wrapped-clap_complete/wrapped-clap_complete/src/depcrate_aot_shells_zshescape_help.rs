// Generated macro for escape_help (function)
macro_rules! Depcrate_aot_shells_zshescape_help {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"escape_help"}
// Dependencies: {}
# [doc = " Escape help string inside single quotes and brackets"] fn escape_help (string : & str) -> String { string . replace ('\\' , "\\\\") . replace ('\'' , "'\\''") . replace ('[' , "\\[") . replace (']' , "\\]") . replace (':' , "\\:") . replace ('$' , "\\$") . replace ('`' , "\\`") . replace ('\n' , " ") }
};
}
