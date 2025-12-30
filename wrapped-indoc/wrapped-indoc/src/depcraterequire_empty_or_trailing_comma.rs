// Generated macro for require_empty_or_trailing_comma (function)
macro_rules! Depcraterequire_empty_or_trailing_comma {
() => {
// Module: crate
// Provides: {"require_empty_or_trailing_comma"}
// Dependencies: {}
fn require_empty_or_trailing_comma (input : & mut Peekable < TokenIter >) -> Result < () > { let first = match input . next () { Some (TokenTree :: Punct (punct)) if punct . as_char () == ',' => match input . next () { Some (second) => second , None => return Ok (()) , } , Some (first) => first , None => return Ok (()) , } ; let last = input . last () ; let begin_span = first . span () ; let end_span = last . as_ref () . map_or (begin_span , TokenTree :: span) ; let msg = format ! ("unexpected {token} in macro invocation; indoc argument must be a single string literal" , token = if last . is_some () { "tokens" } else { "token" }) ; Err (Error :: new2 (begin_span , end_span , & msg)) }
};
}
