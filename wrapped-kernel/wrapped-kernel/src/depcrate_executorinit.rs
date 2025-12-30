// Generated macro for init (function)
macro_rules! Depcrate_executorinit {
() => {
// Module: crate::executor
// Provides: {"init"}
// Dependencies: {}
pub fn init () { # [cfg (feature = "net")] crate :: executor :: network :: init () ; # [cfg (feature = "vsock")] crate :: executor :: vsock :: init () ; # [cfg (feature = "alloc-stats")] crate :: executor :: alloc_stats :: init () ; }
};
}
