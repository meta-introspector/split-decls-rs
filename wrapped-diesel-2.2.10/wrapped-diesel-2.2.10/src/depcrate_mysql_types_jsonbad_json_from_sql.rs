// Generated macro for bad_json_from_sql (function)
macro_rules! Depcrate_mysql_types_jsonbad_json_from_sql {
() => {
// Module: crate::mysql::types::json
// Provides: {"bad_json_from_sql"}
// Dependencies: {}
# [test] fn bad_json_from_sql () { use crate :: mysql :: MysqlType ; let uuid : Result < serde_json :: Value , _ > = FromSql :: < sql_types :: Json , Mysql > :: from_sql (MysqlValue :: new_internal (b"boom" , MysqlType :: String) ,) ; assert_eq ! (uuid . unwrap_err () . to_string () , "Invalid Json") ; }
};
}
