// Generated macro for SerdeJsonValueProxy (struct)
macro_rules! Depcrate_type_impls_jsonSerdeJsonValueProxy {
() => {
// Module: crate::type_impls::json
// Provides: {"SerdeJsonValueProxy"}
// Dependencies: {}
# [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Json)] # [cfg_attr (feature = "postgres_backend" , diesel (sql_type = Jsonb))] struct SerdeJsonValueProxy (serde_json :: Value) ;
};
}
