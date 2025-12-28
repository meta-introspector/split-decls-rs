macro_rules! expand_cfg {
    () => {
        pub (crate) fn expand_cfg (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; ExpandResult :: Ready (match parse_cfg (cx , sp , tts) { Ok (cfg) => { let matches_cfg = attr :: cfg_matches (& cfg , & cx . sess , cx . current_expansion . lint_node_id , Some (cx . ecfg . features) ,) ; MacEager :: expr (cx . expr_bool (sp , matches_cfg)) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
    };
}

expand_cfg!()