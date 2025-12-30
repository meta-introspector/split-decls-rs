// Generated macro for impl_2212 (impl)
macro_rules! Depcrate_mysql_backendimpl_2212 {
() => {
// Module: crate::mysql::backend
// Provides: {"impl_2212"}
// Dependencies: {}
impl Backend for Mysql { type QueryBuilder = MysqlQueryBuilder ; type RawValue < 'a > = MysqlValue < 'a > ; type BindCollector < 'a > = RawBytesBindCollector < Self > ; }
};
}
