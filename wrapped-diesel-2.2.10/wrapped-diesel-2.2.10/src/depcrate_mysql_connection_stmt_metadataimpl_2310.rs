// Generated macro for impl_2310 (impl)
macro_rules! Depcrate_mysql_connection_stmt_metadataimpl_2310 {
() => {
// Module: crate::mysql::connection::stmt::metadata
// Provides: {"impl_2310"}
// Dependencies: {}
impl MysqlFieldMetadata < '_ > { pub (in crate :: mysql :: connection) fn field_name (& self) -> Option < & str > { if self . 0 . name . is_null () { None } else { unsafe { Some (CStr :: from_ptr (self . 0 . name) . to_str () . expect ("Expect mysql field names to be UTF-8, because we \
                     requested UTF-8 encoding on connection setup" ,)) } } } pub (in crate :: mysql :: connection) fn field_type (& self) -> ffi :: enum_field_types { self . 0 . type_ } pub (in crate :: mysql :: connection) fn flags (& self) -> Flags { Flags :: from (self . 0 . flags) } }
};
}
