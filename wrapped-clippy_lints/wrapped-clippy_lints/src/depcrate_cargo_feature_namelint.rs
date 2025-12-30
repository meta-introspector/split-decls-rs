// Generated macro for lint (function)
macro_rules! Depcrate_cargo_feature_namelint {
() => {
// Module: crate::cargo::feature_name
// Provides: {"lint"}
// Dependencies: {}
fn lint (cx : & LateContext < '_ > , feature : & str , substring : & str , is_prefix : bool) { let is_negative = is_prefix && is_negative_prefix (substring) ; span_lint_and_help (cx , if is_negative { NEGATIVE_FEATURE_NAMES } else { REDUNDANT_FEATURE_NAMES } , DUMMY_SP , format ! ("the \"{substring}\" {} in the feature name \"{feature}\" is {}" , if is_prefix { "prefix" } else { "suffix" } , if is_negative { "negative" } else { "redundant" }) , None , format ! ("consider renaming the feature to \"{}\"{}" , if is_prefix { feature . strip_prefix (substring) } else { feature . strip_suffix (substring) } . unwrap () , if is_negative { ", but make sure the feature adds functionality" } else { "" }) ,) ; }
};
}
