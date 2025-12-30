// Generated macro for get_len (function)
macro_rules! Depcrate_dataget_len {
() => {
// Module: crate::data
// Provides: {"get_len"}
// Dependencies: {}
fn get_len (bytes : & [u8]) -> CFIndex { let len = bytes . len () ; debug_assert ! (len < CFIndex :: MAX as usize) ; len as CFIndex }
};
}
