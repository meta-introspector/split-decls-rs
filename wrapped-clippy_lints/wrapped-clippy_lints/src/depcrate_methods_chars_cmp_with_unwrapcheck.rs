// Generated macro for check (function)
macro_rules! Depcrate_methods_chars_cmp_with_unwrapcheck {
() => {
// Module: crate::methods::chars_cmp_with_unwrap
// Provides: {"check"}
// Dependencies: {}
# [doc = " Wrapper fn for `CHARS_NEXT_CMP` and `CHARS_LAST_CMP` lints with `unwrap()`."] pub (super) fn check (cx : & LateContext < '_ > , info : & crate :: methods :: BinaryExprInfo < '_ > , chain_methods : & [Symbol] , lint : & 'static Lint , suggest : & str ,) -> bool { if let Some (args) = method_chain_args (info . chain , chain_methods) && let hir :: ExprKind :: Lit (lit) = info . other . kind && let ast :: LitKind :: Char (c) = lit . node { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , lint , info . expr . span , format ! ("you should use the `{suggest}` method") , "like this" , format ! ("{}{}.{suggest}('{}')" , if info . eq { "" } else { "!" } , snippet_with_applicability (cx , args [0] . 0 . span , ".." , & mut applicability) , c . escape_default ()) , applicability ,) ; true } else { false } }
};
}
