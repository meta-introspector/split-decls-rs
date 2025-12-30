// Generated macro for impl_3339 (impl)
macro_rules! Depcrate_pg_metadata_lookupimpl_3339 {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"impl_3339"}
// Dependencies: {}
impl PgMetadataCache { # [doc = " Construct a new `PgMetadataCache`"] pub fn new () -> Self { Default :: default () } # [doc = " Lookup the OID of a custom type"] pub fn lookup_type (& self , type_name : & PgMetadataCacheKey < '_ >) -> Option < PgTypeMetadata > { Some (PgTypeMetadata (Ok (* self . cache . get (type_name) ?))) } # [doc = " Store the OID of a custom type"] pub fn store_type (& mut self , type_name : PgMetadataCacheKey < '_ > , type_metadata : impl Into < InnerPgTypeMetadata > ,) { self . cache . insert (type_name . into_owned () , type_metadata . into ()) ; } }
};
}
