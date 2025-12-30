// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1306 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1306"}
// Dependencies: {}
impl < DB , Inner > QueryFragment < DB > for SqlQuery < Inner > where DB : Backend + DieselReserveSpecialization , Inner : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; self . inner . walk_ast (out . reborrow ()) ? ; out . push_sql (& self . query) ; Ok (()) } }
};
}
