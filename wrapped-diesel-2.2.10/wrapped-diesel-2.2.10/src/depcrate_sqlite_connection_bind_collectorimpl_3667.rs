// Generated macro for impl_3667 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3667 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3667"}
// Dependencies: {}
impl MoveableBindCollector < Sqlite > for SqliteBindCollector < '_ > { type BindData = SqliteBindCollectorData ; fn moveable (& self) -> Self :: BindData { let mut binds = Vec :: with_capacity (self . binds . len ()) ; for b in self . binds . iter () . map (| (bind , tpe) | (OwnedSqliteBindValue :: from (bind) , * tpe)) { binds . push (b) ; } SqliteBindCollectorData { binds } } fn append_bind_data (& mut self , from : & Self :: BindData) { self . binds . reserve_exact (from . binds . len ()) ; self . binds . extend (from . binds . iter () . map (| (bind , tpe) | (InternalSqliteBindValue :: from (bind) , * tpe)) ,) ; } }
};
}
