// Generated macro for TEST_CONFIG_NO_SNIPPET_CAP (const)
macro_rules! Depcrate_testsTEST_CONFIG_NO_SNIPPET_CAP {
() => {
// Module: crate::tests
// Provides: {"TEST_CONFIG_NO_SNIPPET_CAP"}
// Dependencies: {}
pub (crate) const TEST_CONFIG_NO_SNIPPET_CAP : AssistConfig = AssistConfig { snippet_cap : None , allowed : None , insert_use : InsertUseConfig { granularity : ImportGranularity :: Crate , prefix_kind : hir :: PrefixKind :: Plain , enforce_granularity : true , group : true , skip_glob_imports : true , } , prefer_no_std : false , prefer_prelude : true , prefer_absolute : false , assist_emit_must_use : false , term_search_fuel : 400 , term_search_borrowck : true , code_action_grouping : true , expr_fill_default : ExprFillDefaultMode :: Todo , prefer_self_ty : false , } ;
};
}
