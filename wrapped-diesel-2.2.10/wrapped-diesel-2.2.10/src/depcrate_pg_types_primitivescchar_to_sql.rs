// Generated macro for cchar_to_sql (function)
macro_rules! Depcrate_pg_types_primitivescchar_to_sql {
() => {
// Module: crate::pg::types::primitives
// Provides: {"cchar_to_sql"}
// Dependencies: {}
# [test] fn cchar_to_sql () { use crate :: query_builder :: bind_collector :: ByteWrapper ; let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < sql_types :: CChar , Pg > :: to_sql (& b'A' , & mut bytes) . unwrap () ; ToSql :: < sql_types :: CChar , Pg > :: to_sql (& b'\xc4' , & mut bytes) . unwrap () ; assert_eq ! (buffer , vec ! [65u8 , 196u8]) ; }
};
}
