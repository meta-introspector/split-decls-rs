// Generated macro for escape_value (function)
macro_rules! Depcrate_aot_shells_zshescape_value {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"escape_value"}
// Dependencies: {}
# [doc = " Escape value string inside single quotes and parentheses"] fn escape_value (string : & str) -> String { string . replace ('\\' , "\\\\") . replace ('\'' , "'\\''") . replace ('[' , "\\[") . replace (']' , "\\]") . replace (':' , "\\:") . replace ('$' , "\\$") . replace ('`' , "\\`") . replace ('(' , "\\(") . replace (')' , "\\)") . replace (' ' , "\\ ") }
};
}
