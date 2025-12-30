// Generated macro for impl_1320 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1320 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1320"}
// Dependencies: {}
impl < DB , Query > QueryFragment < DB > for BoxedSqlQuery < '_ , DB , Query > where DB : Backend + DieselReserveSpecialization , Query : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; self . query . walk_ast (out . reborrow ()) ? ; out . push_sql (& self . sql) ; for b in & self . binds { b . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
