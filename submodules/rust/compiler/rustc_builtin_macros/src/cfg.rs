mkuse!{use rustc_ast :: token ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_errors :: PResult ;}
mkuse!{use rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacEager , MacroExpanderResult } ;}
mkuse!{use rustc_parse :: exp ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use { rustc_ast as ast , rustc_attr_parsing as attr } ;}
mkuse!{use crate :: errors ;}

macro_rules! expand_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_cfg in module {}", module_path!());
    };
}

mkfn!{
    expand_cfg_introspect!();
    pub (crate) fn expand_cfg (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; ExpandResult :: Ready (match parse_cfg (cx , sp , tts) { Ok (cfg) => { let matches_cfg = attr :: cfg_matches (& cfg , & cx . sess , cx . current_expansion . lint_node_id , Some (cx . ecfg . features) ,) ; MacEager :: expr (cx . expr_bool (sp , matches_cfg)) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}

macro_rules! parse_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_introspect!();
    fn parse_cfg < 'a > (cx : & ExtCtxt < 'a > , span : Span , tts : TokenStream ,) -> PResult < 'a , ast :: MetaItemInner > { let mut p = cx . new_parser_from_tts (tts) ; if p . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: RequiresCfgPattern { span })) ; } let cfg = p . parse_meta_item_inner () ? ; let _ = p . eat (exp ! (Comma)) ; if ! p . eat (exp ! (Eof)) { return Err (cx . dcx () . create_err (errors :: OneCfgPattern { span })) ; } Ok (cfg) }
}