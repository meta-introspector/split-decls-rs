// Generated macro for version (function)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
pub fn version () -> & 'static str { env ! ("CARGO_PKG_VERSION") }
};
}
