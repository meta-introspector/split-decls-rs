mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_attr_parsing as attr ;}
mkuse!{use rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;}
mkuse!{use rustc_parse :: parser :: cfg_select :: { CfgSelectBranches , CfgSelectPredicate , parse_cfg_select } ;}
mkuse!{use rustc_span :: { Ident , Span , sym } ;}
mkuse!{use crate :: errors :: { CfgSelectNoMatches , CfgSelectUnreachable } ;}

macro_rules! select_arm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function select_arm in module {}", module_path!());
    };
}

mkfn!{
    select_arm_introspect!();
    # [doc = " Selects the first arm whose predicate evaluates to true."] fn select_arm (ecx : & ExtCtxt < '_ > , branches : CfgSelectBranches) -> Option < (TokenStream , Span) > { for (cfg , tt , arm_span) in branches . reachable { if attr :: cfg_matches (& cfg , & ecx . sess , ecx . current_expansion . lint_node_id , Some (ecx . ecfg . features) ,) { return Some ((tt , arm_span)) ; } } branches . wildcard . map (| (_ , tt , span) | (tt , span)) }
}

macro_rules! expand_cfg_select_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_cfg_select in module {}", module_path!());
    };
}

mkfn!{
    expand_cfg_select_introspect!();
    pub (super) fn expand_cfg_select < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_cfg_select (& mut ecx . new_parser_from_tts (tts)) { Ok (branches) => { if let Some ((underscore , _ , _)) = branches . wildcard { for (predicate , _ , _) in & branches . unreachable { let span = match predicate { CfgSelectPredicate :: Wildcard (underscore) => underscore . span , CfgSelectPredicate :: Cfg (cfg) => cfg . span () , } ; let err = CfgSelectUnreachable { span , wildcard_span : underscore . span } ; ecx . dcx () . emit_warn (err) ; } } if let Some ((tts , arm_span)) = select_arm (ecx , branches) { return ExpandResult :: from_tts (ecx , tts , sp , arm_span , Ident :: with_dummy_span (sym :: cfg_select) ,) ; } else { let guar = ecx . dcx () . emit_err (CfgSelectNoMatches { span : sp }) ; DummyResult :: any (sp , guar) } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}