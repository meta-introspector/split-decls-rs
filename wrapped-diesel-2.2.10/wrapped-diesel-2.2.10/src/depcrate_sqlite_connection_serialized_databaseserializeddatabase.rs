// Generated macro for SerializedDatabase (struct)
macro_rules! Depcrate_sqlite_connection_serialized_databaseSerializedDatabase {
() => {
// Module: crate::sqlite::connection::serialized_database
// Provides: {"SerializedDatabase"}
// Dependencies: {}
# [doc = " `SerializedDatabase` is a wrapper for a serialized database that is dynamically allocated by calling `sqlite3_serialize`."] # [doc = " This RAII wrapper is necessary to deallocate the memory when it goes out of scope with `sqlite3_free`."] # [derive (Debug)] pub struct SerializedDatabase { data : * mut u8 , len : usize , }
};
}
