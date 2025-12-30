// Generated macro for gate (macro)
macro_rules! Depcrate_feature_gategate {
() => {
// Module: crate::feature_gate
// Provides: {"gate"}
// Dependencies: {}
# [doc = " The common case."] macro_rules ! gate { ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , sym ::$ feature , $ span , $ explain) . emit () ; } } } ; ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr , $ help : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , sym ::$ feature , $ span , $ explain) . with_help ($ help) . emit () ; } } } ; }
};
}
