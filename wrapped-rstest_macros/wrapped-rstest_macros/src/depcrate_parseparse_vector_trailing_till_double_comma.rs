// Generated macro for parse_vector_trailing_till_double_comma (function)
macro_rules! Depcrate_parseparse_vector_trailing_till_double_comma {
() => {
// Module: crate::parse
// Provides: {"parse_vector_trailing_till_double_comma"}
// Dependencies: {}
fn parse_vector_trailing_till_double_comma < T , P > (input : ParseStream) -> syn :: Result < Vec < T > > where T : Parse , P : syn :: token :: Token + Parse , { Ok (Punctuated :: < Option < T > , P > :: parse_separated_nonempty_with (input , | input_tokens | { if input_tokens . is_empty () || input_tokens . peek (Token ! [::]) { Ok (None) } else { T :: parse (input_tokens) . map (Some) } }) ? . into_iter () . flatten () . collect () ,) }
};
}
