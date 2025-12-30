// Generated macro for parser_of (function)
macro_rules! Depcrate_aot_shells_zshparser_of {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"parser_of"}
// Dependencies: {}
fn parser_of < 'cmd > (parent : & 'cmd Command , bin_name : & str) -> Option < & 'cmd Command > { debug ! ("parser_of: p={}, bin_name={}" , parent . get_name () , bin_name) ; if bin_name == parent . get_bin_name () . unwrap_or_default () { return Some (parent) ; } for subcommand in parent . get_subcommands () { if let Some (ret) = parser_of (subcommand , bin_name) { return Some (ret) ; } } None }
};
}
