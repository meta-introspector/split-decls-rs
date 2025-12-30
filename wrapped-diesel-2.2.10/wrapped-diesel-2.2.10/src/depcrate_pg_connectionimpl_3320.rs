// Generated macro for impl_3320 (impl)
macro_rules! Depcrate_pg_connectionimpl_3320 {
() => {
// Module: crate::pg::connection
// Provides: {"impl_3320"}
// Dependencies: {}
impl MultiConnectionHelper for PgConnection { fn to_any < 'a > (lookup : & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup ,) -> & mut (dyn std :: any :: Any + 'a) { lookup . as_any () } fn from_any (lookup : & mut dyn std :: any :: Any ,) -> Option < & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup > { lookup . downcast_mut :: < Self > () . map (| conn | conn as & mut dyn super :: PgMetadataLookup) } }
};
}
