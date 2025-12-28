macro_rules! gate_cfg {
    () => {
        # [allow (rustc :: untranslatable_diagnostic)] fn gate_cfg (gated_cfg : & GatedCfg , cfg_span : Span , sess : & Session , features : & Features) { let (cfg , feature , has_feature) = gated_cfg ; if ! has_feature (features) && ! cfg_span . allows_unstable (* feature) { let explain = format ! ("`cfg({cfg})` is experimental and subject to change") ; feature_err (sess , * feature , cfg_span , explain) . emit () ; } }
    };
}

gate_cfg!()