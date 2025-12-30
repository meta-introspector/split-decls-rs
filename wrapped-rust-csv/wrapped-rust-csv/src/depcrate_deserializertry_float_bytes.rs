// Generated macro for try_float_bytes (function)
macro_rules! Depcrate_deserializertry_float_bytes {
() => {
// Module: crate::deserializer
// Provides: {"try_float_bytes"}
// Dependencies: {}
fn try_float_bytes (s : & [u8]) -> Option < f64 > { str :: from_utf8 (s) . ok () . and_then (| s | s . parse () . ok ()) }
};
}
