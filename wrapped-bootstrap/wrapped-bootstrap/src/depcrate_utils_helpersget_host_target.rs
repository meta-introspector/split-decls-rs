// Generated macro for get_host_target (function)
macro_rules! Depcrate_utils_helpersget_host_target {
() => {
// Module: crate::utils::helpers
// Provides: {"get_host_target"}
// Dependencies: {}
# [doc = " Return the host target on which we are currently running."] pub fn get_host_target () -> TargetSelection { TargetSelection :: from_user (env ! ("BUILD_TRIPLE")) }
};
}
