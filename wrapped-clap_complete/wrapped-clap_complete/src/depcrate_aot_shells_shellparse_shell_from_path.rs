// Generated macro for parse_shell_from_path (function)
macro_rules! Depcrate_aot_shells_shellparse_shell_from_path {
() => {
// Module: crate::aot::shells::shell
// Provides: {"parse_shell_from_path"}
// Dependencies: {}
fn parse_shell_from_path (path : & Path) -> Option < Shell > { let name = path . file_stem () ? . to_str () ? ; match name { "bash" => Some (Shell :: Bash) , "zsh" => Some (Shell :: Zsh) , "fish" => Some (Shell :: Fish) , "elvish" => Some (Shell :: Elvish) , "powershell" | "powershell_ise" => Some (Shell :: PowerShell) , _ => None , } }
};
}
