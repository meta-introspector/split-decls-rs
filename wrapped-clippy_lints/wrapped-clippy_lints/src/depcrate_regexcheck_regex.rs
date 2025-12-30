// Generated macro for check_regex (function)
macro_rules! Depcrate_regexcheck_regex {
() => {
// Module: crate::regex
// Provides: {"check_regex"}
// Dependencies: {}
fn check_regex < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , utf8 : bool) { let mut parser = regex_syntax :: ParserBuilder :: new () . unicode (true) . utf8 (utf8) . build () ; if let ExprKind :: Lit (lit) = expr . kind { if let LitKind :: Str (ref r , style) = lit . node { let r = r . as_str () ; let offset = if let StrStyle :: Raw (n) = style { 2 + n } else { 1 } ; match parser . parse (r) { Ok (r) => { if let Some (repl) = is_trivial_regex (& r) { span_lint_and_help (cx , TRIVIAL_REGEX , expr . span , "trivial regex" , None , repl) ; } } , Err (e) => lint_syntax_error (cx , & e , r , expr . span , offset) , } } } else if let Some (r) = const_str (cx , expr) { match parser . parse (& r) { Ok (r) => { if let Some (repl) = is_trivial_regex (& r) { span_lint_and_help (cx , TRIVIAL_REGEX , expr . span , "trivial regex" , None , repl) ; } } , Err (e) => span_lint (cx , INVALID_REGEX , expr . span , e . to_string ()) , } } }
};
}
