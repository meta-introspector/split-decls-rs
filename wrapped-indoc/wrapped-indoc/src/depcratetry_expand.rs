// Generated macro for try_expand (function)
macro_rules! Depcratetry_expand {
() => {
// Module: crate
// Provides: {"try_expand"}
// Dependencies: {}
fn try_expand (input : TokenStream , mode : Macro) -> Result < TokenStream > { let mut input = input . into_iter () . peekable () ; let prefix = match mode { Macro :: Indoc | Macro :: Format | Macro :: Print | Macro :: Eprint => None , Macro :: Write => { let require_comma = true ; let mut expr = expr :: parse (& mut input , require_comma) ? ; expr . extend (iter :: once (input . next () . unwrap ())) ; Some (expr) } Macro :: Concat => return do_concat (input) , } ; let first = input . next () . ok_or_else (| | { Error :: new (Span :: call_site () , "unexpected end of macro invocation, expected format string" ,) }) ? ; let preserve_empty_first_line = false ; let unindented_lit = lit_indoc (first , mode , preserve_empty_first_line) ? ; let macro_name = match mode { Macro :: Indoc => { require_empty_or_trailing_comma (& mut input) ? ; return Ok (TokenStream :: from (TokenTree :: Literal (unindented_lit))) ; } Macro :: Format => "format" , Macro :: Print => "print" , Macro :: Eprint => "eprint" , Macro :: Write => "write" , Macro :: Concat => unreachable ! () , } ; Ok (TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new (macro_name , Span :: call_site ())) , TokenTree :: Punct (Punct :: new ('!' , Spacing :: Alone)) , TokenTree :: Group (Group :: new (Delimiter :: Brace , prefix . unwrap_or_else (TokenStream :: new) . into_iter () . chain (iter :: once (TokenTree :: Literal (unindented_lit))) . chain (input) . collect () ,)) ,])) }
};
}
