// Generated macro for impl_8 (impl)
macro_rules! Depcrate_capabilitiesimpl_8 {
() => {
// Module: crate::capabilities
// Provides: {"impl_8"}
// Dependencies: {}
# [cfg (all (unix , not (target_os = "macos")))] impl Default for Capabilities { fn default () -> Self { Capabilities { precompose_unicode : false , ignore_case : false , executable_bit : true , symlink : true , } } }
};
}
