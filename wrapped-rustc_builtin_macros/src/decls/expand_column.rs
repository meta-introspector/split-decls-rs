macro_rules! expand_column {
    () => {
        # [doc = " Expand `column!()` to the current column number."] pub (crate) fn expand_column (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; check_zero_tts (cx , sp , tts , "column!") ; let topmost = cx . expansion_cause () . unwrap_or (sp) ; let loc = cx . source_map () . lookup_char_pos (topmost . lo ()) ; ExpandResult :: Ready (MacEager :: expr (cx . expr_u32 (topmost , loc . col . to_usize () as u32 + 1))) }
    };
}

expand_column!()