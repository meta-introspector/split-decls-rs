// Generated macro for HasSqlType (trait)
macro_rules! Depcrate_sql_typesHasSqlType {
() => {
// Module: crate::sql_types
// Provides: {"HasSqlType"}
// Dependencies: {}
# [doc = " Indicates that a SQL type exists for a backend."] # [doc = ""] # [doc = " This trait can be derived using the [`SqlType` derive](derive@SqlType)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[derive(diesel::sql_types::SqlType)]"] # [doc = " #[diesel(postgres_type(oid = 23, array_oid = 1007))]"] # [doc = " #[diesel(sqlite_type(name = \"Integer\"))]"] # [doc = " #[diesel(mysql_type(name = \"Long\"))]"] # [doc = " pub struct Integer;"] # [doc = " ```"] pub trait HasSqlType < ST > : TypeMetadata { # [doc = " Fetch the metadata for the given type"] # [doc = ""] # [doc = " This method may use `lookup` to do dynamic runtime lookup. Implementors"] # [doc = " of this method should not do dynamic lookup unless absolutely necessary"] fn metadata (lookup : & mut Self :: MetadataLookup) -> Self :: TypeMetadata ; }
};
}
