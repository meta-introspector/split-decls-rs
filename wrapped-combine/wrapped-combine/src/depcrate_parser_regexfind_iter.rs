// Generated macro for find_iter (function)
macro_rules! Depcrate_parser_regexfind_iter {
() => {
// Module: crate::parser::regex
// Provides: {"find_iter"}
// Dependencies: {}
fn find_iter < 'a , Input , F > (iterable : Input) -> (usize , F) where Input : IntoIterator , Input :: Item : MatchFind , F : FromIterator < < Input :: Item as MatchFind > :: Range > , { let mut end = 0 ; let value = iterable . into_iter () . map (| m | { end = m . end () ; m . as_match () }) . collect () ; (end , value) }
};
}
