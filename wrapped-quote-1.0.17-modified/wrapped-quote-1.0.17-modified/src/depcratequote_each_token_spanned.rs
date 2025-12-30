// Generated macro for quote_each_token_spanned (macro)
macro_rules! Depcratequote_each_token_spanned {
() => {
// Module: crate
// Provides: {"quote_each_token_spanned"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! quote_each_token_spanned { ($ tokens : ident $ span : ident $ ($ tts : tt) *) => { $ crate :: quote_tokens_with_context_spanned ! ($ tokens $ span (@ @ @ @ @ @ $ ($ tts) *) (@ @ @ @ @ $ ($ tts) * @) (@ @ @ @ $ ($ tts) * @ @) (@ @ @ $ (($ tts)) * @ @ @) (@ @ $ ($ tts) * @ @ @ @) (@ $ ($ tts) * @ @ @ @ @) ($ ($ tts) * @ @ @ @ @ @)) ; } ; }
};
}
