// Generated macro for impl_609 (impl)
macro_rules! Depcrate_types_serde_jsonimpl_609 {
() => {
// Module: crate::types::serde_json
// Provides: {"impl_609"}
// Dependencies: {}
# [doc = " Deserialize SQLite value to JSON `Value`:"] # [doc = ""] # [doc = " | SQLite   | JSON    |"] # [doc = " |----------|---------|"] # [doc = " | NULL     | Null    |"] # [doc = " | 'null'   | Null    |"] # [doc = " | 'true'   | Bool    |"] # [doc = " | 1        | Number  |"] # [doc = " | 0.1      | Number  |"] # [doc = " | '\"text\"' | String  |"] # [doc = " | 'text'   | _Error_ |"] # [doc = " | '[0, 1]' | Array   |"] # [doc = " | '{\"x\": 1}' | Object  |"] impl FromSql for Value { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Text (s) => serde_json :: from_slice (s) , ValueRef :: Blob (b) => serde_json :: from_slice (b) , ValueRef :: Integer (i) => Ok (Self :: Number (Number :: from (i))) , ValueRef :: Real (f) => { match Number :: from_f64 (f) { Some (n) => Ok (Self :: Number (n)) , _ => return Err (FromSqlError :: InvalidType) , } } ValueRef :: Null => Ok (Self :: Null) , } . map_err (FromSqlError :: other) } }
};
}
