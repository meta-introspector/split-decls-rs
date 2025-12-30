// Generated macro for impl_2364 (impl)
macro_rules! Depcrate_mysql_connectionimpl_2364 {
() => {
// Module: crate::mysql::connection
// Provides: {"impl_2364"}
// Dependencies: {}
# [cfg (feature = "r2d2")] impl crate :: r2d2 :: R2D2Connection for MysqlConnection { fn ping (& mut self) -> QueryResult < () > { crate :: r2d2 :: CheckConnectionQuery . execute (self) . map (| _ | ()) } fn is_broken (& mut self) -> bool { AnsiTransactionManager :: is_broken_transaction_manager (self) } }
};
}
