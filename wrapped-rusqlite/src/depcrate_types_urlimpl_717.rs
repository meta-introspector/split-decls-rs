// Generated macro for impl_717 (impl)
macro_rules! Depcrate_types_urlimpl_717 {
() => {
// Module: crate::types::url
// Provides: {"impl_717"}
// Dependencies: {}
# [doc = " Serialize `Url` to text."] impl ToSql for Url { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_str ())) } }
};
}
