// Generated macro for try_positive_integer64_bytes (function)
macro_rules! Depcrate_deserializertry_positive_integer64_bytes {
() => {
// Module: crate::deserializer
// Provides: {"try_positive_integer64_bytes"}
// Dependencies: {}
fn try_positive_integer64_bytes (s : & [u8]) -> Option < u64 > { str :: from_utf8 (s) . ok () . and_then (| s | s . parse () . ok ()) }
};
}
