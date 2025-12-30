// Generated macro for get_toolchain_rustflags (function)
macro_rules! Depcrateget_toolchain_rustflags {
() => {
// Module: crate
// Provides: {"get_toolchain_rustflags"}
// Dependencies: {}
fn get_toolchain_rustflags (name : & str) -> String { format ! ("--cfg __ZEROCOPY_TOOLCHAIN=\"{}\" " , name) }
};
}
