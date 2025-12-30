// Generated macro for pasted_to_tokens (function)
macro_rules! Depcratepasted_to_tokens {
() => {
// Module: crate
// Provides: {"pasted_to_tokens"}
// Dependencies: {}
fn pasted_to_tokens (mut pasted : String , span : Span) -> Result < TokenStream > { let mut tokens = TokenStream :: new () ; # [cfg (not (no_literal_fromstr))] { use proc_macro :: { LexError , Literal } ; use std :: str :: FromStr ; if pasted . starts_with (| ch : char | ch . is_ascii_digit ()) { let literal = match panic :: catch_unwind (| | Literal :: from_str (& pasted)) { Ok (Ok (literal)) => TokenTree :: Literal (literal) , Ok (Err (LexError { .. })) | Err (_) => { return Err (Error :: new (span , & format ! ("`{:?}` is not a valid literal" , pasted) ,)) ; } } ; tokens . extend (iter :: once (literal)) ; return Ok (tokens) ; } } if pasted . starts_with ('\'') { let mut apostrophe = TokenTree :: Punct (Punct :: new ('\'' , Spacing :: Joint)) ; apostrophe . set_span (span) ; tokens . extend (iter :: once (apostrophe)) ; pasted . remove (0) ; } let ident = match panic :: catch_unwind (| | Ident :: new (& pasted , span)) { Ok (ident) => TokenTree :: Ident (ident) , Err (_) => { return Err (Error :: new (span , & format ! ("`{:?}` is not a valid identifier" , pasted) ,)) ; } } ; tokens . extend (iter :: once (ident)) ; Ok (tokens) }
};
}
