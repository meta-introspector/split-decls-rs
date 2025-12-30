// Generated macro for BindData (struct)
macro_rules! Depcrate_mysql_connection_bindBindData {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"BindData"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct BindData { tpe : ffi :: enum_field_types , bytes : Option < NonNull < u8 > > , length : libc :: c_ulong , capacity : usize , flags : Flags , is_null : ffi :: my_bool , is_truncated : Option < ffi :: my_bool > , }
};
}
