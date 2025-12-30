// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Errno > for io :: Error { fn from (errno : Errno) -> Self { io :: Error :: from_raw_os_error (errno . 0) } }
};
}
