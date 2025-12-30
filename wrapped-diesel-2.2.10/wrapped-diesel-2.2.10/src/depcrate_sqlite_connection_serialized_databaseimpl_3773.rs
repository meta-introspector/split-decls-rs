// Generated macro for impl_3773 (impl)
macro_rules! Depcrate_sqlite_connection_serialized_databaseimpl_3773 {
() => {
// Module: crate::sqlite::connection::serialized_database
// Provides: {"impl_3773"}
// Dependencies: {}
impl SerializedDatabase { # [doc = " Creates a new `SerializedDatabase` with the given data pointer and length."] # [doc = ""] # [doc = " SAFETY: The data pointer needs to be returned by sqlite"] # [doc = "         and the length must match the underlying buffer pointer"] pub (crate) unsafe fn new (data : * mut u8 , len : usize) -> Self { Self { data , len } } # [doc = " Returns a slice of the serialized database."] pub fn as_slice (& self) -> & [u8] { unsafe { std :: slice :: from_raw_parts (self . data , self . len) } } }
};
}
