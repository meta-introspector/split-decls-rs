// Generated macro for parse_list_terminated_pattern (function)
macro_rules! Depcrate_matches_patternparse_list_terminated_pattern {
() => {
// Module: crate::matches_pattern
// Provides: {"parse_list_terminated_pattern"}
// Dependencies: {}
# [doc = " Returns the parsed struct pattern body along with a `..` if it appears at"] # [doc = " the end of the body."] # [doc = ""] # [doc = " This is like `Punctuated::parse_terminated`, but additionally allows for an"] # [doc = " optional `..`, which cannot be followed by a comma."] fn parse_list_terminated_pattern < T : Parse > (input : ParseStream < '_ > ,) -> syn :: Result < (Vec < T > , Option < DotDot >) > { let mut patterns = vec ! [] ; while ! input . is_empty () { let dot_dot = input . parse :: < Option < Token ! [..] > > () ? ; if dot_dot . is_some () { return if input . is_empty () { Ok ((patterns , dot_dot)) } else { compile_err (input . span () , "`..` must be at the end of the struct pattern") } ; } patterns . push (input . parse :: < T > () ?) ; if input . is_empty () { break ; } input . parse :: < Token ! [,] > () ? ; } Ok ((patterns , None)) }
};
}
