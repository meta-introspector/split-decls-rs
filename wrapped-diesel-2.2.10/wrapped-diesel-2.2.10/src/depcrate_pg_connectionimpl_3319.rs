// Generated macro for impl_3319 (impl)
macro_rules! Depcrate_pg_connectionimpl_3319 {
() => {
// Module: crate::pg::connection
// Provides: {"impl_3319"}
// Dependencies: {}
# [cfg (feature = "r2d2")] impl crate :: r2d2 :: R2D2Connection for PgConnection { fn ping (& mut self) -> QueryResult < () > { crate :: r2d2 :: CheckConnectionQuery . execute (self) . map (| _ | ()) } fn is_broken (& mut self) -> bool { AnsiTransactionManager :: is_broken_transaction_manager (self) } }
};
}
