// Generated macro for fetch_bytes (function)
macro_rules! Depcrate_exportfetch_bytes {
() => {
// Module: crate::export
// Provides: {"fetch_bytes"}
// Dependencies: {}
# [doc = " Get and clear the logged bytes"] # [cfg (feature = "unstable-test")] pub fn fetch_bytes () -> Vec < u8 > { BYTES . with (| b | core :: mem :: take (& mut * b . borrow_mut ())) }
};
}
