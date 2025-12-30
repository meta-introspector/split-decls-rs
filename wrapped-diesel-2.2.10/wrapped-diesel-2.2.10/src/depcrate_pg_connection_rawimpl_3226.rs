// Generated macro for impl_3226 (impl)
macro_rules! Depcrate_pg_connection_rawimpl_3226 {
() => {
// Module: crate::pg::connection::raw
// Provides: {"impl_3226"}
// Dependencies: {}
impl From < PGTransactionStatusType > for PgTransactionStatus { fn from (trans_status_type : PGTransactionStatusType) -> Self { match trans_status_type { PGTransactionStatusType :: PQTRANS_IDLE => PgTransactionStatus :: Idle , PGTransactionStatusType :: PQTRANS_ACTIVE => PgTransactionStatus :: Active , PGTransactionStatusType :: PQTRANS_INTRANS => PgTransactionStatus :: InTransaction , PGTransactionStatusType :: PQTRANS_INERROR => PgTransactionStatus :: InError , PGTransactionStatusType :: PQTRANS_UNKNOWN => PgTransactionStatus :: Unknown , } } }
};
}
