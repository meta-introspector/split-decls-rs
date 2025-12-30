// Generated macro for impl_37 (impl)
macro_rules! Depcrate_sqliteimpl_37 {
() => {
// Module: crate::sqlite
// Provides: {"impl_37"}
// Dependencies: {}
impl Type < Sqlite > for Timestamp { fn type_info () -> SqliteTypeInfo { < str as Type < Sqlite > > :: type_info () } fn compatible (ty : & SqliteTypeInfo) -> bool { < str as Type < Sqlite > > :: compatible (ty) || < f64 as Type < Sqlite > > :: compatible (ty) } }
};
}
