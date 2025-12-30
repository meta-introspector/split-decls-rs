// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl DiagnosticsConfig { pub fn test_sample () -> Self { use hir :: PrefixKind ; use ide_db :: imports :: insert_use :: ImportGranularity ; Self { enabled : true , proc_macros_enabled : Default :: default () , proc_attr_macros_enabled : Default :: default () , disable_experimental : Default :: default () , disabled : Default :: default () , expr_fill_default : Default :: default () , style_lints : true , snippet_cap : SnippetCap :: new (true) , insert_use : InsertUseConfig { granularity : ImportGranularity :: Preserve , enforce_granularity : false , prefix_kind : PrefixKind :: Plain , group : false , skip_glob_imports : false , } , prefer_no_std : false , prefer_prelude : true , prefer_absolute : false , term_search_fuel : 400 , term_search_borrowck : true , } } }
};
}
