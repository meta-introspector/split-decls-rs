// Generated macro for tests (module)
macro_rules! Depcrate_pg_types_integerstests {
() => {
// Module: crate::pg::types::integers
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: query_builder :: bind_collector :: ByteWrapper ; # [test] fn i16_to_sql () { let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < sql_types :: SmallInt , Pg > :: to_sql (& 1i16 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: SmallInt , Pg > :: to_sql (& 0i16 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: SmallInt , Pg > :: to_sql (& - 1i16 , & mut bytes) . unwrap () ; assert_eq ! (buffer , vec ! [0 , 1 , 0 , 0 , 255 , 255]) ; } # [test] fn i32_to_sql () { let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < sql_types :: Integer , Pg > :: to_sql (& 1i32 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: Integer , Pg > :: to_sql (& 0i32 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: Integer , Pg > :: to_sql (& - 1i32 , & mut bytes) . unwrap () ; assert_eq ! (buffer , vec ! [0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 255 , 255 , 255 , 255]) ; } # [test] fn i64_to_sql () { let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& 1i64 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& 0i64 , & mut bytes) . unwrap () ; ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& - 1i64 , & mut bytes) . unwrap () ; assert_eq ! (buffer , vec ! [0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 ,]) ; } }
};
}
