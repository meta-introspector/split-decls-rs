// Generated macro for impl_2247 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2247 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2247"}
// Dependencies: {}
impl Drop for BindData { fn drop (& mut self) { if let Some (bytes) = self . bytes { std :: mem :: drop (unsafe { Vec :: from_raw_parts (bytes . as_ptr () , 0 , self . capacity) }) ; self . bytes = None ; } } }
};
}
