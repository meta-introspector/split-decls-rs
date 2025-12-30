// Generated macro for impl_584 (impl)
macro_rules! Depcrate_types_from_sqlimpl_584 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_584"}
// Dependencies: {}
impl < const N : usize > FromSql for [u8 ; N] { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let slice = value . as_blob () ? ; slice . try_into () . map_err (| _ | FromSqlError :: InvalidBlobSize { expected_size : N , blob_size : slice . len () , }) } }
};
}
