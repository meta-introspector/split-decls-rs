// Generated macro for gate_legacy (macro)
macro_rules! Depcrate_feature_gategate_legacy {
() => {
// Module: crate::feature_gate
// Provides: {"gate_legacy"}
// Dependencies: {}
# [doc = " The legacy case."] macro_rules ! gate_legacy { ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { feature_warn (&$ visitor . sess , sym ::$ feature , $ span , $ explain) ; } } } ; }
};
}
