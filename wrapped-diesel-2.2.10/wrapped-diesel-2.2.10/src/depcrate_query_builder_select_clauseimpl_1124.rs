// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1124 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1124"}
// Dependencies: {}
impl < QS : AsQuerySource > DefaultSelectClause < QS > { pub (crate) fn new (qs : & QS) -> Self { Self { default_selection : qs . as_query_source () . default_selection () , } } }
};
}
