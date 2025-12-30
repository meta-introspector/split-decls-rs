// Generated macro for bool_to_sql (function)
macro_rules! Depcrate_pg_types_primitivesbool_to_sql {
() => {
// Module: crate::pg::types::primitives
// Provides: {"bool_to_sql"}
// Dependencies: {}
# [test] fn bool_to_sql () { use crate :: query_builder :: bind_collector :: ByteWrapper ; let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < sql_types :: Bool , Pg > :: to_sql (& true , & mut bytes) . unwrap () ; ToSql :: < sql_types :: Bool , Pg > :: to_sql (& false , & mut bytes) . unwrap () ; assert_eq ! (buffer , vec ! [1u8 , 0u8]) ; }
};
}
