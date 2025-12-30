// Generated macro for check_command_line (function)
macro_rules! Depcrate_attrs_blanket_clippy_restriction_lintscheck_command_line {
() => {
// Module: crate::attrs::blanket_clippy_restriction_lints
// Provides: {"check_command_line"}
// Dependencies: {}
pub (super) fn check_command_line (cx : & EarlyContext < '_ >) { for (name , level) in & cx . sess () . opts . lint_opts { if name == "clippy::restriction" && * level > Level :: Allow { span_lint_and_then (cx , BLANKET_CLIPPY_RESTRICTION_LINTS , DUMMY_SP , "`clippy::restriction` is not meant to be enabled as a group" , | diag | { diag . note (format ! ("because of the command line `--{} clippy::restriction`" , level . as_str ())) ; diag . help ("enable the restriction lints you need individually") ; } ,) ; } } }
};
}
