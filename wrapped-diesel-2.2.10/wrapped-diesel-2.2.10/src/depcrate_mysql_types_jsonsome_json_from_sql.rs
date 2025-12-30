// Generated macro for some_json_from_sql (function)
macro_rules! Depcrate_mysql_types_jsonsome_json_from_sql {
() => {
// Module: crate::mysql::types::json
// Provides: {"some_json_from_sql"}
// Dependencies: {}
# [test] fn some_json_from_sql () { use crate :: mysql :: MysqlType ; let input_json = b"true" ; let output_json : serde_json :: Value = FromSql :: < sql_types :: Json , Mysql > :: from_sql (MysqlValue :: new_internal (input_json , MysqlType :: String) ,) . unwrap () ; assert_eq ! (output_json , serde_json :: Value :: Bool (true)) ; }
};
}
