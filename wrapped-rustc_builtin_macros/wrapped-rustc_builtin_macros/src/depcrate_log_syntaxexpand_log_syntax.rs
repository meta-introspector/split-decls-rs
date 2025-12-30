// Generated macro for expand_log_syntax (function)
macro_rules! Depcrate_log_syntaxexpand_log_syntax {
() => {
// Module: crate::log_syntax
// Provides: {"expand_log_syntax"}
// Dependencies: {}
pub (crate) fn expand_log_syntax < 'cx > (_cx : & 'cx mut ExtCtxt < '_ > , sp : rustc_span :: Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { println ! ("{}" , pprust :: tts_to_string (& tts)) ; ExpandResult :: Ready (DummyResult :: any_valid (sp)) }
};
}
