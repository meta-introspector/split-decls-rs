// Generated macro for impl_1892 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1892 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1892"}
// Dependencies: {}
impl < From > ToInnerJoin for SelectStatement < FromClause < From > > where From : ToInnerJoin + QuerySource , From :: InnerJoin : QuerySource , { type InnerJoin = SelectStatement < FromClause < From :: InnerJoin > > ; }
};
}
