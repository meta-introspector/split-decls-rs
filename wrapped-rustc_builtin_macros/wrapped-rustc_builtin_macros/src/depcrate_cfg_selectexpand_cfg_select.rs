// Generated macro for expand_cfg_select (function)
macro_rules! Depcrate_cfg_selectexpand_cfg_select {
() => {
// Module: crate::cfg_select
// Provides: {"expand_cfg_select"}
// Dependencies: {}
pub (super) fn expand_cfg_select < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_cfg_select (& mut ecx . new_parser_from_tts (tts)) { Ok (branches) => { if let Some ((underscore , _ , _)) = branches . wildcard { for (predicate , _ , _) in & branches . unreachable { let span = match predicate { CfgSelectPredicate :: Wildcard (underscore) => underscore . span , CfgSelectPredicate :: Cfg (cfg) => cfg . span () , } ; let err = CfgSelectUnreachable { span , wildcard_span : underscore . span } ; ecx . dcx () . emit_warn (err) ; } } if let Some ((tts , arm_span)) = select_arm (ecx , branches) { return ExpandResult :: from_tts (ecx , tts , sp , arm_span , Ident :: with_dummy_span (sym :: cfg_select) ,) ; } else { let guar = ecx . dcx () . emit_err (CfgSelectNoMatches { span : sp }) ; DummyResult :: any (sp , guar) } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
};
}
