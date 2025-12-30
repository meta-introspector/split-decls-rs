// Generated macro for impl_3694 (impl)
macro_rules! Depcrate_sqlite_connection_functionsimpl_3694 {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"impl_3694"}
// Dependencies: {}
impl FunctionRow < '_ > { # [allow (unsafe_code)] fn new (args : & mut [* mut ffi :: sqlite3_value]) -> Self { let lengths = args . len () ; let args = unsafe { Vec :: from_raw_parts (args as * mut [* mut ffi :: sqlite3_value] as * mut ffi :: sqlite3_value as * mut Option < OwnedSqliteValue > , lengths , lengths ,) } ; Self { field_count : lengths , args : Rc :: new (RefCell :: new (ManuallyDrop :: new (PrivateSqliteRow :: Duplicated { values : args , column_names : Rc :: from (vec ! [None ; lengths]) , } ,))) , marker : PhantomData , } } }
};
}
