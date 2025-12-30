// Generated macro for PgMetadataCacheKey (struct)
macro_rules! Depcrate_pg_metadata_lookupPgMetadataCacheKey {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"PgMetadataCacheKey"}
// Dependencies: {}
# [doc = " The key used to lookup cached type oid's inside of"] # [doc = " a [PgMetadataCache]."] # [derive (Hash , PartialEq , Eq , Debug , Clone)] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] pub struct PgMetadataCacheKey < 'a > { pub (in crate :: pg) schema : Option < Cow < 'a , str > > , pub (in crate :: pg) type_name : Cow < 'a , str > , }
};
}
