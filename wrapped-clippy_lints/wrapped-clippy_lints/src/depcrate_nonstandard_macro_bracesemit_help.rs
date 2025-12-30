// Generated macro for emit_help (function)
macro_rules! Depcrate_nonstandard_macro_bracesemit_help {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"emit_help"}
// Dependencies: {}
fn emit_help (cx : & EarlyContext < '_ > , snip : & str , (open , close) : (char , char) , span : Span , add_semi : bool) { let semi = if add_semi { ";" } else { "" } ; if let Some ((macro_name , macro_args_str)) = snip . split_once ('!') { let mut macro_args = macro_args_str . trim () . to_string () ; macro_args . pop () ; macro_args . remove (0) ; span_lint_and_sugg (cx , NONSTANDARD_MACRO_BRACES , span , format ! ("use of irregular braces for `{macro_name}!` macro") , "consider writing" , format ! ("{macro_name}!{open}{macro_args}{close}{semi}") , Applicability :: MachineApplicable ,) ; } }
};
}
