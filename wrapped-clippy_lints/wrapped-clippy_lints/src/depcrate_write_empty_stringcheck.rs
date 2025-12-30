// Generated macro for check (function)
macro_rules! Depcrate_write_empty_stringcheck {
() => {
// Module: crate::write::empty_string
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , format_args : & FormatArgs , macro_call : & MacroCall , name : & str) { if let [FormatArgsPiece :: Literal (sym :: LF)] = & format_args . template [..] { let mut span = format_args . span ; let lint = if name == "writeln" { span = expand_past_previous_comma (cx , span) ; WRITELN_EMPTY_STRING } else { PRINTLN_EMPTY_STRING } ; span_lint_and_then (cx , lint , macro_call . span , format ! ("empty string literal in `{name}!`") , | diag | { diag . span_suggestion (span , "remove the empty string" , String :: new () , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
