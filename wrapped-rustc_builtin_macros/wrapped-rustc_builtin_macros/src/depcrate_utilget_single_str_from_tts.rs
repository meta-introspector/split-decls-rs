// Generated macro for get_single_str_from_tts (function)
macro_rules! Depcrate_utilget_single_str_from_tts {
() => {
// Module: crate::util
// Provides: {"get_single_str_from_tts"}
// Dependencies: {}
# [doc = " Interpreting `tts` as a comma-separated sequence of expressions,"] # [doc = " expect exactly one string literal, or emit an error and return `Err`."] pub (crate) fn get_single_str_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < Symbol , ErrorGuaranteed > , () > { get_single_str_spanned_from_tts (cx , span , tts , name) . map (| res | res . map (| (s , _) | s)) }
};
}
