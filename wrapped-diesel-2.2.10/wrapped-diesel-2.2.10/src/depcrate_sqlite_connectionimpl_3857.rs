// Generated macro for impl_3857 (impl)
macro_rules! Depcrate_sqlite_connectionimpl_3857 {
() => {
// Module: crate::sqlite::connection
// Provides: {"impl_3857"}
// Dependencies: {}
# [cfg (feature = "r2d2")] impl crate :: r2d2 :: R2D2Connection for crate :: sqlite :: SqliteConnection { fn ping (& mut self) -> QueryResult < () > { use crate :: RunQueryDsl ; crate :: r2d2 :: CheckConnectionQuery . execute (self) . map (| _ | ()) } fn is_broken (& mut self) -> bool { AnsiTransactionManager :: is_broken_transaction_manager (self) } }
};
}
