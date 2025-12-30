// Generated macro for expand_include_bytes (function)
macro_rules! Depcrate_source_utilexpand_include_bytes {
() => {
// Module: crate::source_util
// Provides: {"expand_include_bytes"}
// Dependencies: {}
# [doc = " Expand `include_bytes!($input)` to the content of the file given by path `$input`."] # [doc = ""] # [doc = " This works in expression, pattern and statement position."] pub (crate) fn expand_include_bytes (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; let ExpandResult :: Ready (mac) = get_single_str_spanned_from_tts (cx , sp , tts , "include_bytes!") else { return ExpandResult :: Retry (()) ; } ; let (path , path_span) = match mac { Ok (res) => res , Err (guar) => return ExpandResult :: Ready (DummyResult :: any (sp , guar)) , } ; ExpandResult :: Ready (match load_binary_file (cx , path . as_str () . as_ref () , sp , path_span) { Ok ((bytes , _bsp)) => { let expr = cx . expr (sp , ast :: ExprKind :: IncludedBytes (ByteSymbol :: intern (& bytes))) ; MacEager :: expr (expr) } Err (dummy) => dummy , }) }
};
}
