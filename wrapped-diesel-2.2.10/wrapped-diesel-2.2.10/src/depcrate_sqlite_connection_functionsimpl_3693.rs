// Generated macro for impl_3693 (impl)
macro_rules! Depcrate_sqlite_connection_functionsimpl_3693 {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"impl_3693"}
// Dependencies: {}
impl Drop for FunctionRow < '_ > { # [allow (unsafe_code)] fn drop (& mut self) { if let Some (args) = Rc :: get_mut (& mut self . args) { if let PrivateSqliteRow :: Duplicated { column_names , .. } = DerefMut :: deref_mut (RefCell :: get_mut (args)) { if Rc :: strong_count (column_names) == 1 { unsafe { std :: ptr :: drop_in_place (column_names as * mut _) } } } } } }
};
}
