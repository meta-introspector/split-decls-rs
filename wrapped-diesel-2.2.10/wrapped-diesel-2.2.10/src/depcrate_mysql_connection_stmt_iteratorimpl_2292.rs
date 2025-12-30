// Generated macro for impl_2292 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2292 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2292"}
// Dependencies: {}
impl PrivateMysqlRow { fn duplicate (& self) -> Self { match self { Self :: Copied (b) | Self :: Direct (b) => Self :: Copied (b . clone ()) , } } }
};
}
