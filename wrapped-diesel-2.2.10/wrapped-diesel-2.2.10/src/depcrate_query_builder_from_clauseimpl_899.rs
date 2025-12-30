// Generated macro for impl_899 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_899 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_899"}
// Dependencies: {}
impl < QS1 , QS2 > AppearsInFromClause < QS1 > for FromClause < QS2 > where QS1 : QuerySource , QS2 : QuerySource , QS2 : AppearsInFromClause < QS1 > , { type Count = < QS2 as AppearsInFromClause < QS1 > > :: Count ; }
};
}
