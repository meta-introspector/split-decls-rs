// Generated macro for impl_3697 (impl)
macro_rules! Depcrate_sqlite_connection_functionsimpl_3697 {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"impl_3697"}
// Dependencies: {}
impl RowIndex < usize > for FunctionRow < '_ > { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count () { Some (idx) } else { None } } }
};
}
