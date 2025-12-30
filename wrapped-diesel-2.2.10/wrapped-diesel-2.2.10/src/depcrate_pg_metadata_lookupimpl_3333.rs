// Generated macro for impl_3333 (impl)
macro_rules! Depcrate_pg_metadata_lookupimpl_3333 {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"impl_3333"}
// Dependencies: {}
impl < T > PgMetadataLookup for T where T : Connection < Backend = Pg > + GetPgMetadataCache + LoadConnection < DefaultLoadingMode > , { fn lookup_type (& mut self , type_name : & str , schema : Option < & str >) -> PgTypeMetadata { let cache_key = PgMetadataCacheKey { schema : schema . map (Cow :: Borrowed) , type_name : Cow :: Borrowed (type_name) , } ; { let metadata_cache = self . get_metadata_cache () ; if let Some (metadata) = metadata_cache . lookup_type (& cache_key) { return metadata ; } } let r = lookup_type (& cache_key , self) ; match r { Ok (type_metadata) => { self . get_metadata_cache () . store_type (cache_key , type_metadata) ; PgTypeMetadata (Ok (type_metadata)) } Err (_e) => PgTypeMetadata (Err (FailedToLookupTypeError :: new_internal (cache_key . into_owned () ,))) , } } fn as_any < 'a > (& mut self) -> & mut (dyn std :: any :: Any + 'a) where Self : 'a , { self } }
};
}
