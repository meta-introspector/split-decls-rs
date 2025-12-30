// Generated macro for impl_3337 (impl)
macro_rules! Depcrate_pg_metadata_lookupimpl_3337 {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"impl_3337"}
// Dependencies: {}
impl < 'a > PgMetadataCacheKey < 'a > { # [doc = " Construct a new cache key from an optional schema name and"] # [doc = " a type name"] pub fn new (schema : Option < Cow < 'a , str > > , type_name : Cow < 'a , str >) -> Self { Self { schema , type_name } } # [doc = " Convert the possibly borrowed version of this metadata cache key"] # [doc = " into a lifetime independent owned version"] pub fn into_owned (self) -> PgMetadataCacheKey < 'static > { let PgMetadataCacheKey { schema , type_name } = self ; PgMetadataCacheKey { schema : schema . map (| s | Cow :: Owned (s . into_owned ())) , type_name : Cow :: Owned (type_name . into_owned ()) , } } }
};
}
