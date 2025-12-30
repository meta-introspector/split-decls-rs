// Generated macro for json_to_sql (function)
macro_rules! Depcrate_mysql_types_jsonjson_to_sql {
() => {
// Module: crate::mysql::types::json
// Provides: {"json_to_sql"}
// Dependencies: {}
# [test] fn json_to_sql () { use crate :: query_builder :: bind_collector :: ByteWrapper ; let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let test_json = serde_json :: Value :: Bool (true) ; ToSql :: < sql_types :: Json , Mysql > :: to_sql (& test_json , & mut bytes) . unwrap () ; assert_eq ! (buffer , b"true") ; }
};
}
