macro_rules! deps {
    () => {
        CfgMatchesLintEmitter!();
    };
}

macro_rules! cfg_matches {
    () => {
        deps!();
        # [doc = " Tests if a cfg-pattern matches the cfg set"] pub fn cfg_matches (cfg : & MetaItemInner , sess : & Session , lint_emitter : impl CfgMatchesLintEmitter , features : Option < & Features > ,) -> bool { eval_condition (cfg , sess , features , & mut | cfg | { try_gate_cfg (cfg . name , cfg . span , sess , features) ; match sess . psess . check_config . expecteds . get (& cfg . name) { Some (ExpectedValues :: Some (values)) if ! values . contains (& cfg . value) => { lint_emitter . emit_span_lint (sess , UNEXPECTED_CFGS , cfg . span , BuiltinLintDiag :: UnexpectedCfgValue ((cfg . name , cfg . name_span) , cfg . value . map (| v | (v , cfg . value_span . unwrap ())) ,) ,) ; } None if sess . psess . check_config . exhaustive_names => { lint_emitter . emit_span_lint (sess , UNEXPECTED_CFGS , cfg . span , BuiltinLintDiag :: UnexpectedCfgName ((cfg . name , cfg . name_span) , cfg . value . map (| v | (v , cfg . value_span . unwrap ())) ,) ,) ; } _ => { } } sess . psess . config . contains (& (cfg . name , cfg . value)) }) }
    };
}

cfg_matches!()