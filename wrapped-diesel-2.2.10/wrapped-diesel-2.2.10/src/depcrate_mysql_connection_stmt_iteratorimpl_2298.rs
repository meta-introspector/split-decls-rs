// Generated macro for impl_2298 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2298 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2298"}
// Dependencies: {}
impl < 'a > Field < 'a , Mysql > for MysqlField < 'a > { fn field_name (& self) -> Option < & str > { self . metadata . fields () [self . idx] . field_name () } fn is_null (& self) -> bool { match & * self . binds { PrivateMysqlRow :: Copied (b) | PrivateMysqlRow :: Direct (b) => b [self . idx] . is_null () , } } fn value (& self) -> Option < < Mysql as Backend > :: RawValue < '_ > > { match & * self . binds { PrivateMysqlRow :: Copied (b) | PrivateMysqlRow :: Direct (b) => b [self . idx] . value () , } } }
};
}
