// Generated macro for parse_comma (function)
macro_rules! Depcrate_helpersparse_comma {
() => {
// Module: crate::helpers
// Provides: {"parse_comma"}
// Dependencies: {}
# [doc = " Consume a comma, returning a `TokenStream` describing the error upon failure."] pub (crate) fn parse_comma (iter : & mut impl Iterator < Item = TokenTree >) -> Result < () , TokenStream > { match iter . next () { Some (TokenTree :: Punct (punct)) if punct . as_char () == ',' => Ok (()) , Some (TokenTree :: Punct (punct)) => { let first_span = punct . span () ; let last_span = iter . take_while (| token | matches ! (token , TokenTree :: Punct (_))) . last () . map_or (first_span , | token | token . span ()) ; Err (compile_error ("minimum and maximum value must be separated by a comma" , (first_span , last_span) ,)) } Some (token) => Err (compile_error ("minimum and maximum value must be separated by a comma" , token . span () ,)) , None => Err (compile_error ("expected maximum value" , None)) , } }
};
}
