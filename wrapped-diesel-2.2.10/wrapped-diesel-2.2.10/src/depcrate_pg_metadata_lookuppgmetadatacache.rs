// Generated macro for PgMetadataCache (struct)
macro_rules! Depcrate_pg_metadata_lookupPgMetadataCache {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"PgMetadataCache"}
// Dependencies: {}
# [doc = " Cache for the [OIDs] of custom Postgres types"] # [doc = ""] # [doc = " [OIDs]: https://www.postgresql.org/docs/current/static/datatype-oid.html"] # [allow (missing_debug_implementations)] # [derive (Default)] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] pub struct PgMetadataCache { cache : HashMap < PgMetadataCacheKey < 'static > , InnerPgTypeMetadata > , }
};
}
