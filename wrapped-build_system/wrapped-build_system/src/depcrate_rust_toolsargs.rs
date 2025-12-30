// Generated macro for args (function)
macro_rules! Depcrate_rust_toolsargs {
() => {
// Module: crate::rust_tools
// Provides: {"args"}
// Dependencies: {}
fn args (command : & str) -> Result < Option < Vec < String > > , String > { if let Some ("--help") = std :: env :: args () . nth (2) . as_deref () { usage (command) ; return Ok (None) ; } let args = std :: env :: args () . skip (2) . collect :: < Vec < _ > > () ; if args . is_empty () { return Err (format ! ("Expected at least one argument for `{command}` subcommand, found none")) ; } Ok (Some (args)) }
};
}
