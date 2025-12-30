// Generated macro for impl_20 (impl)
macro_rules! Depcrate_matches_patternimpl_20 {
() => {
// Module: crate::matches_pattern
// Provides: {"impl_20"}
// Dependencies: {}
impl ParsedMatchPattern { fn into_matcher_expr (self) -> TokenStream { let Self { struct_name , group } = self ; let mut first_err = None ; if let Some (ref g) = group { let res = match g . delimiter () { Delimiter :: Parenthesis => parse_tuple_pattern_args (struct_name . clone () , g . stream ()) , Delimiter :: Brace => parse_braced_pattern_args (struct_name . clone () , g . stream ()) , Delimiter :: Bracket => compile_err (g . span () , "[...] syntax is not meaningful") , Delimiter :: None => compile_err (g . span () , "undelimited group not supported") , } ; match res { Ok (res) => return res , Err (e) => first_err = Some (e) , } } into_match_pattern_expr (quote ! { # struct_name # group }) . map_err (| e | first_err . unwrap_or (e)) . unwrap_or_else (syn :: Error :: into_compile_error) } }
};
}
