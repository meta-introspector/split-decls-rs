// Generated macro for known_buffer_size_for_ffi_type (function)
macro_rules! Depcrate_mysql_connection_bindknown_buffer_size_for_ffi_type {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"known_buffer_size_for_ffi_type"}
// Dependencies: {}
fn known_buffer_size_for_ffi_type (tpe : ffi :: enum_field_types) -> Option < usize > { use self :: ffi :: enum_field_types as t ; use std :: mem :: size_of ; match tpe { t :: MYSQL_TYPE_TINY => Some (1) , t :: MYSQL_TYPE_YEAR | t :: MYSQL_TYPE_SHORT => Some (2) , t :: MYSQL_TYPE_INT24 | t :: MYSQL_TYPE_LONG | t :: MYSQL_TYPE_FLOAT => Some (4) , t :: MYSQL_TYPE_LONGLONG | t :: MYSQL_TYPE_DOUBLE => Some (8) , t :: MYSQL_TYPE_TIME | t :: MYSQL_TYPE_DATE | t :: MYSQL_TYPE_DATETIME | t :: MYSQL_TYPE_TIMESTAMP => Some (size_of :: < MysqlTime > ()) , _ => None , } }
};
}
