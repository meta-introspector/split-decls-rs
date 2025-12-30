// Generated macro for FunctionRow (struct)
macro_rules! Depcrate_sqlite_connection_functionsFunctionRow {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"FunctionRow"}
// Dependencies: {}
struct FunctionRow < 'a > { args : Rc < RefCell < ManuallyDrop < PrivateSqliteRow < 'a , 'static > > > > , field_count : usize , marker : PhantomData < & 'a ffi :: sqlite3_value > , }
};
}
