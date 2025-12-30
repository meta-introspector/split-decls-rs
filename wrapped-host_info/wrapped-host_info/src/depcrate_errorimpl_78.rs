// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (target_os = "windows")] impl From < windows :: core :: Error > for HostInfoError { fn from (input : windows :: core :: Error) -> Self { Self :: Windows (input) } }
};
}
