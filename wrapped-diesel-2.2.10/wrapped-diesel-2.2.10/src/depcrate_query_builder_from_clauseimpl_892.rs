// Generated macro for impl_892 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_892 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_892"}
// Dependencies: {}
impl < F > AsQuerySource for FromClause < F > where F : QuerySource , { type QuerySource = F ; fn as_query_source (& self) -> & Self :: QuerySource { & self . source } }
};
}
