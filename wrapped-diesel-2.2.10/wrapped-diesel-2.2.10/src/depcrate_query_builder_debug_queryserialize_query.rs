// Generated macro for serialize_query (function)
macro_rules! Depcrate_query_builder_debug_queryserialize_query {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"serialize_query"}
// Dependencies: {}
fn serialize_query < DB > (query : & dyn QueryFragment < DB >) -> Result < String , fmt :: Error > where DB : Backend + Default , DB :: QueryBuilder : Default , { let mut query_builder = DB :: QueryBuilder :: default () ; let backend = DB :: default () ; QueryFragment :: < DB > :: to_sql (query , & mut query_builder , & backend) . map_err (| _ | fmt :: Error) ? ; Ok (query_builder . finish ()) }
};
}
