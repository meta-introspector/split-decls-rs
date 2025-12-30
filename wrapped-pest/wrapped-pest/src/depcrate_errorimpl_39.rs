// Generated macro for impl_39 (impl)
macro_rules! Depcrate_errorimpl_39 {
() => {
// Module: crate::error
// Provides: {"impl_39"}
// Dependencies: {}
impl < R : RuleType > ParseAttempts < R > { # [doc = " Helper formatting function to get message informing about tokens we've"] # [doc = " (un)expected to see."] # [doc = " Used as a part of `parse_attempts_error`."] fn tokens_helper_messages (& self , is_whitespace_fn : & IsWhitespaceFn , spacing : & str ,) -> Vec < String > { let mut helper_messages = Vec :: new () ; let tokens_header_pairs = vec ! [(self . expected_tokens () , "expected") , (self . unexpected_tokens () , "unexpected") ,] ; for (tokens , header) in & tokens_header_pairs { if tokens . is_empty () { continue ; } let mut helper_tokens_message = format ! ("{spacing}note: {header} ") ; helper_tokens_message . push_str (if tokens . len () == 1 { "token: " } else { "one of tokens: " }) ; let expected_tokens_set : BTreeSet < String > = tokens . iter () . map (| token | { if token . is_whitespace (is_whitespace_fn) { String :: from ("WHITESPACE") } else { format ! ("`{}`" , token) } }) . collect () ; helper_tokens_message . push_str (& expected_tokens_set . iter () . cloned () . collect :: < Vec < String > > () . join (", ") ,) ; helper_messages . push (helper_tokens_message) ; } helper_messages } }
};
}
