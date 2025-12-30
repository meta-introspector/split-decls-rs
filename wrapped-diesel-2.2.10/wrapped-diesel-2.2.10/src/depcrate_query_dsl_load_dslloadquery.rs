// Generated macro for LoadQuery (trait)
macro_rules! Depcrate_query_dsl_load_dslLoadQuery {
() => {
// Module: crate::query_dsl::load_dsl
// Provides: {"LoadQuery"}
// Dependencies: {}
# [doc = " The `load` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`RunQueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `load` from generic code."] # [doc = ""] # [doc = " [`RunQueryDsl`]: crate::RunQueryDsl"] pub trait LoadQuery < 'query , Conn , U , B = DefaultLoadingMode > : RunQueryDsl < Conn > { # [doc = " Return type of `LoadQuery::internal_load`"] type RowIter < 'conn > : Iterator < Item = QueryResult < U > > where Conn : 'conn ; # [doc = " Load this query"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] fn internal_load (self , conn : & mut Conn) -> QueryResult < Self :: RowIter < '_ > > ; }
};
}
