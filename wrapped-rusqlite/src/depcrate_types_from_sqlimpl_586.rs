// Generated macro for impl_586 (impl)
macro_rules! Depcrate_types_from_sqlimpl_586 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_586"}
// Dependencies: {}
# [cfg (feature = "uuid")] impl FromSql for uuid :: Uuid { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let bytes = < [u8 ; 16] > :: column_result (value) ? ; Ok (Self :: from_u128 (u128 :: from_be_bytes (bytes))) } }
};
}
