// Generated macro for impl_3858 (impl)
macro_rules! Depcrate_sqlite_connectionimpl_3858 {
() => {
// Module: crate::sqlite::connection
// Provides: {"impl_3858"}
// Dependencies: {}
impl MultiConnectionHelper for SqliteConnection { fn to_any < 'a > (lookup : & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup ,) -> & mut (dyn std :: any :: Any + 'a) { lookup } fn from_any (lookup : & mut dyn std :: any :: Any ,) -> Option < & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup > { lookup . downcast_mut () } }
};
}
