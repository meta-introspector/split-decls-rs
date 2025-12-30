// Generated macro for expand_line (function)
macro_rules! Depcrate_source_utilexpand_line {
() => {
// Module: crate::source_util
// Provides: {"expand_line"}
// Dependencies: {}
# [doc = " Expand `line!()` to the current line number."] pub (crate) fn expand_line (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; check_zero_tts (cx , sp , tts , "line!") ; let topmost = cx . expansion_cause () . unwrap_or (sp) ; let loc = cx . source_map () . lookup_char_pos (topmost . lo ()) ; ExpandResult :: Ready (MacEager :: expr (cx . expr_u32 (topmost , loc . line as u32))) }
};
}
