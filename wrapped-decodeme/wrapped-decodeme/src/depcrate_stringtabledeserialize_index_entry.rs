// Generated macro for deserialize_index_entry (function)
macro_rules! Depcrate_stringtabledeserialize_index_entry {
() => {
// Module: crate::stringtable
// Provides: {"deserialize_index_entry"}
// Dependencies: {}
fn deserialize_index_entry (bytes : & [u8]) -> (StringId , Addr) { (StringId :: new (u64 :: from_le_bytes (bytes [0 .. 8] . try_into () . unwrap ())) , Addr (u64 :: from_le_bytes (bytes [8 .. 16] . try_into () . unwrap ())) ,) }
};
}
