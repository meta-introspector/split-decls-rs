// Generated macro for lookup_type (function)
macro_rules! Depcrate_pg_metadata_lookuplookup_type {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"lookup_type"}
// Dependencies: {}
fn lookup_type < T : Connection < Backend = Pg > + LoadConnection < DefaultLoadingMode > > (cache_key : & PgMetadataCacheKey < '_ > , conn : & mut T ,) -> QueryResult < InnerPgTypeMetadata > { let metadata_query = pg_type :: table . select ((pg_type :: oid , pg_type :: typarray)) ; let metadata = if let Some (schema) = cache_key . schema . as_deref () { metadata_query . inner_join (pg_namespace :: table) . filter (pg_type :: typname . eq (& cache_key . type_name)) . filter (pg_namespace :: nspname . eq (schema)) . first (conn) ? } else { metadata_query . filter (pg_type :: oid . eq (crate :: dsl :: sql ("quote_ident(") . bind :: < crate :: sql_types :: Text , _ > (& cache_key . type_name) . sql (")::regtype::oid")) ,) . first (conn) ? } ; Ok (metadata) }
};
}
