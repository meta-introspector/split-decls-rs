// Generated macro for native (function)
macro_rules! Depcrate_cross_compilenative {
() => {
// Module: crate::cross_compile
// Provides: {"native"}
// Dependencies: {}
# [doc = " The arch triple of the test-running host."] pub fn native () -> & 'static str { env ! ("NATIVE_ARCH") }
};
}
