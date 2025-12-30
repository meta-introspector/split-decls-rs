// Generated macro for quote_each_token (macro)
macro_rules! Depcratequote_each_token {
() => {
// Module: crate
// Provides: {"quote_each_token"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! quote_each_token { ($ tokens : ident $ ($ tts : tt) *) => { $ crate :: quote_tokens_with_context ! { $ tokens (@ @ @ @ @ @ $ ($ tts) *) (@ @ @ @ @ $ ($ tts) * @) (@ @ @ @ $ ($ tts) * @ @) (@ @ @ $ (($ tts)) * @ @ @) (@ @ $ ($ tts) * @ @ @ @) (@ $ ($ tts) * @ @ @ @ @) ($ ($ tts) * @ @ @ @ @ @) } } ; }
};
}
