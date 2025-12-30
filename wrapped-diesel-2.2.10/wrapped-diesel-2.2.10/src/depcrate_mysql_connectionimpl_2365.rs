// Generated macro for impl_2365 (impl)
macro_rules! Depcrate_mysql_connectionimpl_2365 {
() => {
// Module: crate::mysql::connection
// Provides: {"impl_2365"}
// Dependencies: {}
impl MultiConnectionHelper for MysqlConnection { fn to_any < 'a > (lookup : & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup ,) -> & mut (dyn std :: any :: Any + 'a) { lookup } fn from_any (lookup : & mut dyn std :: any :: Any ,) -> Option < & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup > { lookup . downcast_mut () } }
};
}
