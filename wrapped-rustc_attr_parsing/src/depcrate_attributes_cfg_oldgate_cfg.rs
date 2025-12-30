// Generated macro for gate_cfg (function)
macro_rules! Depcrate_attributes_cfg_oldgate_cfg {
() => {
// Module: crate::attributes::cfg_old
// Provides: {"gate_cfg"}
// Dependencies: {}
# [allow (rustc :: untranslatable_diagnostic)] fn gate_cfg (gated_cfg : & GatedCfg , cfg_span : Span , sess : & Session , features : & Features) { let (cfg , feature , has_feature) = gated_cfg ; if ! has_feature (features) && ! cfg_span . allows_unstable (* feature) { let explain = format ! ("`cfg({cfg})` is experimental and subject to change") ; feature_err (sess , * feature , cfg_span , explain) . emit () ; } }
};
}
