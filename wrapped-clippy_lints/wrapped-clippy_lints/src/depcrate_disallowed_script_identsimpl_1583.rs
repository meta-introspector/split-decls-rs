// Generated macro for impl_1583 (impl)
macro_rules! Depcrate_disallowed_script_identsimpl_1583 {
() => {
// Module: crate::disallowed_script_idents
// Provides: {"impl_1583"}
// Dependencies: {}
impl EarlyLintPass for DisallowedScriptIdents { fn check_crate (& mut self , cx : & EarlyContext < '_ > , _ : & ast :: Crate) { let check_disallowed_script_idents = cx . builder . lint_level (DISALLOWED_SCRIPT_IDENTS) . level != Level :: Allow ; if ! check_disallowed_script_idents { return ; } let symbols = cx . sess () . psess . symbol_gallery . symbols . lock () ; let mut symbols : Vec < _ > = symbols . iter () . collect () ; symbols . sort_unstable_by_key (| k | k . 1) ; for & (symbol , & span) in & symbols { let symbol_str = symbol . as_str () ; if ! symbol_str . is_ascii () && let Some (script) = symbol_str . chars () . find_map (| c | { if c . is_ascii () { return None ; } c . script_extension () . iter () . find (| script | ! self . whitelist . contains (script)) }) { span_lint (cx , DISALLOWED_SCRIPT_IDENTS , span , format ! ("identifier `{symbol_str}` has a Unicode script that is not allowed by configuration: {}" , script . full_name ()) ,) ; } } } }
};
}
