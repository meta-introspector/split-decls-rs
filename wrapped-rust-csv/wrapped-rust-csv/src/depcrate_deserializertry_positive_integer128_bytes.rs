// Generated macro for try_positive_integer128_bytes (function)
macro_rules! Depcrate_deserializertry_positive_integer128_bytes {
() => {
// Module: crate::deserializer
// Provides: {"try_positive_integer128_bytes"}
// Dependencies: {}
fn try_positive_integer128_bytes (s : & [u8]) -> Option < u128 > { str :: from_utf8 (s) . ok () . and_then (| s | s . parse () . ok ()) }
};
}
