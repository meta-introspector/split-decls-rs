// Generated macro for impl_3789 (impl)
macro_rules! Depcrate_sqlite_connection_sqlite_valueimpl_3789 {
() => {
// Module: crate::sqlite::connection::sqlite_value
// Provides: {"impl_3789"}
// Dependencies: {}
impl OwnedSqliteValue { pub (super) fn copy_from_ptr (ptr : NonNull < ffi :: sqlite3_value >) -> Option < OwnedSqliteValue > { let tpe = unsafe { ffi :: sqlite3_value_type (ptr . as_ptr ()) } ; if ffi :: SQLITE_NULL == tpe { return None ; } let value = unsafe { ffi :: sqlite3_value_dup (ptr . as_ptr ()) } ; Some (Self { value : NonNull :: new (value) ? , }) } pub (super) fn duplicate (& self) -> OwnedSqliteValue { let value = unsafe { ffi :: sqlite3_value_dup (self . value . as_ptr ()) } ; let value = NonNull :: new (value) . expect ("Sqlite documentation states this returns only null if value is null \
                 or OOM. If you ever see this panic message please open an issue at \
                 https://github.com/diesel-rs/diesel." ,) ; OwnedSqliteValue { value } } }
};
}
