// Generated macro for impl_718 (impl)
macro_rules! Depcrate_types_urlimpl_718 {
() => {
// Module: crate::types::url
// Provides: {"impl_718"}
// Dependencies: {}
# [doc = " Deserialize text to `Url`."] impl FromSql for Url { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Text (s) => { let s = std :: str :: from_utf8 (s) . map_err (FromSqlError :: other) ? ; Self :: parse (s) . map_err (FromSqlError :: other) } _ => Err (FromSqlError :: InvalidType) , } } }
};
}
