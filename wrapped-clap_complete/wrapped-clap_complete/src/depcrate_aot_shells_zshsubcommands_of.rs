// Generated macro for subcommands_of (function)
macro_rules! Depcrate_aot_shells_zshsubcommands_of {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"subcommands_of"}
// Dependencies: {}
fn subcommands_of (p : & Command) -> String { debug ! ("subcommands_of") ; let mut segments = vec ! [] ; fn add_subcommands (subcommand : & Command , name : & str , ret : & mut Vec < String >) { debug ! ("add_subcommands") ; let text = format ! ("'{name}:{help}' \\" , name = name , help = escape_help (& subcommand . get_about () . unwrap_or_default () . to_string ())) ; ret . push (text) ; } for command in p . get_subcommands () { debug ! ("subcommands_of:iter: subcommand={}" , command . get_name ()) ; add_subcommands (command , command . get_name () , & mut segments) ; for alias in command . get_visible_aliases () { add_subcommands (command , alias , & mut segments) ; } } if ! segments . is_empty () { segments . insert (0 , "" . to_string ()) ; segments . push ("    " . to_string ()) ; } segments . join ("\n") }
};
}
