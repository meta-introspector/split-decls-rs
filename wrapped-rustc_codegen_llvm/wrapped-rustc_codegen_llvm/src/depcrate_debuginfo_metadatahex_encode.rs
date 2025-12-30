// Generated macro for hex_encode (function)
macro_rules! Depcrate_debuginfo_metadatahex_encode {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"hex_encode"}
// Dependencies: {}
fn hex_encode (data : & [u8]) -> String { let mut hex_string = String :: with_capacity (data . len () * 2) ; for byte in data . iter () { write ! (& mut hex_string , "{byte:02x}") . unwrap () ; } hex_string }
};
}
