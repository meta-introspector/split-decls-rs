// Generated macro for impl_3393 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3393 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3393"}
// Dependencies: {}
impl PgMetadataLookup for Dummy { fn lookup_type (& mut self , type_name : & str , schema : Option < & str >) -> crate :: pg :: PgTypeMetadata { let cache_key = PgMetadataCacheKey :: new (schema . map (Into :: into) . map (Cow :: Owned) , Cow :: Owned (type_name . into ()) ,) ; crate :: pg :: PgTypeMetadata (Err (FailedToLookupTypeError :: new_internal (cache_key))) } }
};
}
