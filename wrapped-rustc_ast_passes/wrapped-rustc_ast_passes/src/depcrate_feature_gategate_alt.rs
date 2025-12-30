// Generated macro for gate_alt (macro)
macro_rules! Depcrate_feature_gategate_alt {
() => {
// Module: crate::feature_gate
// Provides: {"gate_alt"}
// Dependencies: {}
# [doc = " The unusual case, where the `has_feature` condition is non-standard."] macro_rules ! gate_alt { ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , $ name , $ span , $ explain) . emit () ; } } } ; ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr , $ notes : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] let mut diag = feature_err (&$ visitor . sess , $ name , $ span , $ explain) ; for note in $ notes { diag . note (* note) ; } diag . emit () ; } } } ; }
};
}
