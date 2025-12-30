// Generated macro for impl_585 (impl)
macro_rules! Depcrate_types_from_sqlimpl_585 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_585"}
// Dependencies: {}
# [cfg (feature = "i128_blob")] impl FromSql for i128 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let bytes = < [u8 ; 16] > :: column_result (value) ? ; Ok (Self :: from_be_bytes (bytes) ^ (1_i128 << 127)) } }
};
}
