// Generated macro for check_undefined_nt (function)
macro_rules! Depcrate_grammarcheck_undefined_nt {
() => {
// Module: crate::grammar
// Provides: {"check_undefined_nt"}
// Dependencies: {}
# [doc = " Checks for nonterminals that are used but not defined."] fn check_undefined_nt (grammar : & Grammar , diag : & mut Diagnostics) { grammar . visit_nt (& mut | nt | { if ! grammar . productions . contains_key (nt) { warn_or_err ! (diag , "non-terminal `{nt}` is used but not defined") ; } }) ; }
};
}
