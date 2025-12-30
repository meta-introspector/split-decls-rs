// Generated macro for subcommands (function)
macro_rules! Depcrate_rendersubcommands {
() => {
// Module: crate::render
// Provides: {"subcommands"}
// Dependencies: {}
pub (crate) fn subcommands (roff : & mut Roff , cmd : & clap :: Command , section : & str) { let mut sorted_subcommands : Vec < _ > = cmd . get_subcommands () . filter (| s | ! s . is_hide_set ()) . collect () ; sorted_subcommands . sort_by_key (| c | subcommand_sort_key (c)) ; for sub in sorted_subcommands { roff . control ("TP" , []) ; let name = format ! ("{}-{}({})" , cmd . get_display_name () . unwrap_or_else (|| cmd . get_name ()) , sub . get_name () , section) ; roff . text ([roman (name)]) ; if let Some (about) = sub . get_about () . or_else (| | sub . get_long_about ()) { for line in about . to_string () . lines () { roff . text ([roman (line)]) ; } } } }
};
}
