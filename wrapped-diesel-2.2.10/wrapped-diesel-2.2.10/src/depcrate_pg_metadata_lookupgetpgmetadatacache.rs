// Generated macro for GetPgMetadataCache (trait)
macro_rules! Depcrate_pg_metadata_lookupGetPgMetadataCache {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"GetPgMetadataCache"}
// Dependencies: {}
# [doc = " Gets the `PgMetadataCache` for a `Connection<Backend=Pg>`"] # [doc = " so that the lookup of user defined types, or types which come from an extension can be cached."] # [doc = ""] # [doc = " Implementing this trait for a `Connection<Backend=Pg>` will cause `PgMetadataLookup` to be auto implemented."] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] pub trait GetPgMetadataCache { # [doc = " Get the `PgMetadataCache`"] fn get_metadata_cache (& mut self) -> & mut PgMetadataCache ; }
};
}
