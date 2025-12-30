// Generated macro for impl_3654 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3654 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3654"}
// Dependencies: {}
impl < 'a , T > From < Option < T > > for SqliteBindValue < 'a > where T : Into < SqliteBindValue < 'a > > , { fn from (o : Option < T >) -> Self { match o { Some (v) => v . into () , None => Self { inner : InternalSqliteBindValue :: Null , } , } } }
};
}
