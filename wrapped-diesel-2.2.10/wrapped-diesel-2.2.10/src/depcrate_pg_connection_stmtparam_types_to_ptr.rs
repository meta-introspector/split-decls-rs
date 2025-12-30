// Generated macro for param_types_to_ptr (function)
macro_rules! Depcrate_pg_connection_stmtparam_types_to_ptr {
() => {
// Module: crate::pg::connection::stmt
// Provides: {"param_types_to_ptr"}
// Dependencies: {}
fn param_types_to_ptr (param_types : Option < & Vec < u32 > >) -> * const pq_sys :: Oid { param_types . map (| types | types . as_ptr ()) . unwrap_or (ptr :: null ()) }
};
}
