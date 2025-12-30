// Generated macro for Json (struct)
macro_rules! Depcrate_sql_typesJson {
() => {
// Module: crate::sql_types
// Provides: {"Json"}
// Dependencies: {}
# [doc = " The JSON SQL type.  This type can only be used with `feature ="] # [doc = " \"serde_json\"`"] # [doc = ""] # [doc = " For postgresql you should normally prefer [`Jsonb`](struct.Jsonb.html) instead,"] # [doc = " for the reasons discussed there."] # [doc = ""] # [doc = " ### [`ToSql`] impls"] # [doc = ""] # [doc = " - [`serde_json::Value`]"] # [doc = ""] # [doc = " ### [`FromSql`] impls"] # [doc = ""] # [doc = " - [`serde_json::Value`]"] # [doc = ""] # [doc = " [`ToSql`]: /serialize/trait.ToSql.html"] # [doc = " [`FromSql`]: /deserialize/trait.FromSql.html"] # [doc = " [`serde_json::Value`]: /../serde_json/value/enum.Value.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 114 , array_oid = 199))] # [diesel (mysql_type (name = "String"))] pub struct Json ;
};
}
