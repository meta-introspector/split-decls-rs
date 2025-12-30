// Generated macro for expand_compile_error (function)
macro_rules! Depcrate_compile_errorexpand_compile_error {
() => {
// Module: crate::compile_error
// Provides: {"expand_compile_error"}
// Dependencies: {}
pub (crate) fn expand_compile_error < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let ExpandResult :: Ready (mac) = get_single_str_from_tts (cx , sp , tts , "compile_error!") else { return ExpandResult :: Retry (()) ; } ; let var = match mac { Ok (var) => var , Err (guar) => return ExpandResult :: Ready (DummyResult :: any (sp , guar)) , } ; # [expect (rustc :: diagnostic_outside_of_impl , reason = "diagnostic message is specified by user")] # [expect (rustc :: untranslatable_diagnostic , reason = "diagnostic message is specified by user")] let guar = cx . dcx () . span_err (sp , var . to_string ()) ; ExpandResult :: Ready (DummyResult :: any (sp , guar)) }
};
}
