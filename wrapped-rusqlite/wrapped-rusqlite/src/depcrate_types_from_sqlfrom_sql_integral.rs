// Generated macro for from_sql_integral (macro)
macro_rules! Depcrate_types_from_sqlfrom_sql_integral {
() => {
// Module: crate::types::from_sql
// Provides: {"from_sql_integral"}
// Dependencies: {}
macro_rules ! from_sql_integral (($ t : ident) => (impl FromSql for $ t { # [inline] fn column_result (value : ValueRef <'_ >) -> FromSqlResult < Self > { let i = i64 :: column_result (value) ?; i . try_into () . map_err (| _ | FromSqlError :: OutOfRange (i)) } }) ; (non_zero $ nz : ty , $ z : ty) => (impl FromSql for $ nz { # [inline] fn column_result (value : ValueRef <'_ >) -> FromSqlResult < Self > { let i = <$ z >:: column_result (value) ?; <$ nz >:: new (i) . ok_or (FromSqlError :: OutOfRange (0)) } })) ;
};
}
