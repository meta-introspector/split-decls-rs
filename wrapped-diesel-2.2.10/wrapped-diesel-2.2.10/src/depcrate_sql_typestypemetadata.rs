// Generated macro for TypeMetadata (trait)
macro_rules! Depcrate_sql_typesTypeMetadata {
() => {
// Module: crate::sql_types
// Provides: {"TypeMetadata"}
// Dependencies: {}
# [doc = " Information about how a backend stores metadata about given SQL types"] pub trait TypeMetadata { # [doc = " The actual type used to represent metadata."] # [doc = ""] # [doc = " On PostgreSQL, this is the type's OID."] # [doc = " On MySQL and SQLite, this is an enum representing all storage classes"] # [doc = " they support."] type TypeMetadata ; # [doc = " The type used for runtime lookup of metadata."] # [doc = ""] # [doc = " For most backends, which don't support user defined types, this will"] # [doc = " be `()`."] type MetadataLookup : ? Sized ; }
};
}
