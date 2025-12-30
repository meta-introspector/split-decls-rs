// Generated macro for has_enclosing_paren (function)
macro_rules! Depcrate_sugghas_enclosing_paren {
() => {
// Module: crate::sugg
// Provides: {"has_enclosing_paren"}
// Dependencies: {}
# [doc = " Returns `true` if `sugg` is enclosed in parenthesis."] pub fn has_enclosing_paren (sugg : impl AsRef < str >) -> bool { let mut chars = sugg . as_ref () . chars () ; if chars . next () == Some ('(') { let mut depth = 1 ; for c in & mut chars { if c == '(' { depth += 1 ; } else if c == ')' { depth -= 1 ; } if depth == 0 { break ; } } chars . next () . is_none () } else { false } }
};
}
