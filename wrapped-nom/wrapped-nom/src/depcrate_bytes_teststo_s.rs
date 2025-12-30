// Generated macro for to_s (function)
macro_rules! Depcrate_bytes_teststo_s {
() => {
// Module: crate::bytes::tests
// Provides: {"to_s"}
// Dependencies: {}
# [cfg (feature = "alloc")] fn to_s (i : Vec < u8 >) -> String { String :: from_utf8_lossy (& i) . into_owned () }
};
}
