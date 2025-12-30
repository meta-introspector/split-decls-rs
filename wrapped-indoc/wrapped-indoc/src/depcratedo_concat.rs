// Generated macro for do_concat (function)
macro_rules! Depcratedo_concat {
() => {
// Module: crate
// Provides: {"do_concat"}
// Dependencies: {}
fn do_concat (mut input : Peekable < TokenIter >) -> Result < TokenStream > { let mut result = TokenStream :: new () ; let mut first = true ; while input . peek () . is_some () { let require_comma = false ; let mut expr = expr :: parse (& mut input , require_comma) ? ; let mut expr_tokens = expr . clone () . into_iter () ; if let Some (token) = expr_tokens . next () { if expr_tokens . next () . is_none () { let preserve_empty_first_line = ! first ; if let Ok (literal) = lit_indoc (token , Macro :: Concat , preserve_empty_first_line) { result . extend (iter :: once (TokenTree :: Literal (literal))) ; expr = TokenStream :: new () ; } } } result . extend (expr) ; if let Some (comma) = input . next () { result . extend (iter :: once (comma)) ; } else { break ; } first = false ; } Ok (TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("concat" , Span :: call_site ())) , TokenTree :: Punct (Punct :: new ('!' , Spacing :: Alone)) , TokenTree :: Group (Group :: new (Delimiter :: Brace , result)) ,])) }
};
}
