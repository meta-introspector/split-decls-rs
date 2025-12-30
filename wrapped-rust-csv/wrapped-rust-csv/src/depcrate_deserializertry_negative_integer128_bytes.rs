// Generated macro for try_negative_integer128_bytes (function)
macro_rules! Depcrate_deserializertry_negative_integer128_bytes {
() => {
// Module: crate::deserializer
// Provides: {"try_negative_integer128_bytes"}
// Dependencies: {}
fn try_negative_integer128_bytes (s : & [u8]) -> Option < i128 > { str :: from_utf8 (s) . ok () . and_then (| s | s . parse () . ok ()) }
};
}
