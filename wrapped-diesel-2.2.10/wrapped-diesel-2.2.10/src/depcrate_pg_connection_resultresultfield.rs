// Generated macro for ResultField (enum)
macro_rules! Depcrate_pg_connection_resultResultField {
() => {
// Module: crate::pg::connection::result
// Provides: {"ResultField"}
// Dependencies: {}
# [doc = " Represents valid options to"] # [doc = " [`PQresultErrorField`](https://www.postgresql.org/docs/current/static/libpq-exec.html#LIBPQ-PQRESULTERRORFIELD)"] # [doc = " Their values are defined as C preprocessor macros, and therefore are not exported by libpq-sys."] # [doc = " Their values can be found in `postgres_ext.h`"] # [repr (i32)] enum ResultField { SqlState = 'C' as i32 , MessagePrimary = 'M' as i32 , MessageDetail = 'D' as i32 , MessageHint = 'H' as i32 , TableName = 't' as i32 , ColumnName = 'c' as i32 , ConstraintName = 'n' as i32 , StatementPosition = 'P' as i32 , }
};
}
