// Generated macro for impl_147 (impl)
macro_rules! Depcrate_builtinimpl_147 {
() => {
// Module: crate::builtin
// Provides: {"impl_147"}
// Dependencies: {}
impl EarlyLintPass for IncompleteInternalFeatures { fn check_crate (& mut self , cx : & EarlyContext < '_ > , _ : & ast :: Crate) { let features = cx . builder . features () ; let lang_features = features . enabled_lang_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; let lib_features = features . enabled_lib_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; lang_features . chain (lib_features) . filter (| (name , _) | features . incomplete (* name) || features . internal (* name)) . for_each (| (name , span) | { if features . incomplete (name) { let note = rustc_feature :: find_feature_issue (name , GateIssue :: Language) . map (| n | BuiltinFeatureIssueNote { n }) ; let help = HAS_MIN_FEATURES . contains (& name) . then_some (BuiltinIncompleteFeaturesHelp) ; cx . emit_span_lint (INCOMPLETE_FEATURES , span , BuiltinIncompleteFeatures { name , note , help } ,) ; } else { cx . emit_span_lint (INTERNAL_FEATURES , span , BuiltinInternalFeatures { name }) ; } }) ; } }
};
}
