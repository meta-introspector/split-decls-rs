macro_rules! expand_file {
    () => {
        # [doc = " Expand `file!()` to the current filename."] pub (crate) fn expand_file (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; check_zero_tts (cx , sp , tts , "file!") ; let topmost = cx . expansion_cause () . unwrap_or (sp) ; let loc = cx . source_map () . lookup_char_pos (topmost . lo ()) ; use rustc_session :: RemapFileNameExt ; use rustc_session :: config :: RemapPathScopeComponents ; ExpandResult :: Ready (MacEager :: expr (cx . expr_str (topmost , Symbol :: intern (& loc . file . name . for_scope (cx . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () ,) ,))) }
    };
}

expand_file!()