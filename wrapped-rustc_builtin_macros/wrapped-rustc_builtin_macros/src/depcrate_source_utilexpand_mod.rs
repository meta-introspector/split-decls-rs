// Generated macro for expand_mod (function)
macro_rules! Depcrate_source_utilexpand_mod {
() => {
// Module: crate::source_util
// Provides: {"expand_mod"}
// Dependencies: {}
# [doc = " Expand `module_path!()` to (a textual representation of) the current module path."] pub (crate) fn expand_mod (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; check_zero_tts (cx , sp , tts , "module_path!") ; let mod_path = & cx . current_expansion . module . mod_path ; let string = join_path_idents (mod_path) ; ExpandResult :: Ready (MacEager :: expr (cx . expr_str (sp , Symbol :: intern (& string)))) }
};
}
