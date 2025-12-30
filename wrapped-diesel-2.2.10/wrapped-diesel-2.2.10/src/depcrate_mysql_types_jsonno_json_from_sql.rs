// Generated macro for no_json_from_sql (function)
macro_rules! Depcrate_mysql_types_jsonno_json_from_sql {
() => {
// Module: crate::mysql::types::json
// Provides: {"no_json_from_sql"}
// Dependencies: {}
# [test] fn no_json_from_sql () { let uuid : Result < serde_json :: Value , _ > = FromSql :: < sql_types :: Json , Mysql > :: from_nullable_sql (None) ; assert_eq ! (uuid . unwrap_err () . to_string () , "Unexpected null for non-null column") ; }
};
}
