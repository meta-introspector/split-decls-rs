macro_rules! expand_stringify {
    () => {
        # [doc = " Expand `stringify!($input)`."] pub (crate) fn expand_stringify (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; let s = pprust :: tts_to_string (& tts) ; ExpandResult :: Ready (MacEager :: expr (cx . expr_str (sp , Symbol :: intern (& s)))) }
    };
}

expand_stringify!()