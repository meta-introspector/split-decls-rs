// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqlitetuple_impls {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ Tuple : tt { $ (($ idx : tt) -> $ T : ident , $ ST : ident , $ TT : ident ,) + }) +) => { $ (impl_contains_defaultable_value ! ($ ($ T ,) *) ;) * } }
};
}
