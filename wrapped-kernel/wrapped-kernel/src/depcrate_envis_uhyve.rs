// Generated macro for is_uhyve (function)
macro_rules! Depcrate_envis_uhyve {
() => {
// Module: crate::env
// Provides: {"is_uhyve"}
// Dependencies: {}
# [doc = " Whether Hermit is running under the \"uhyve\" hypervisor."] pub fn is_uhyve () -> bool { matches ! (boot_info () . platform_info , PlatformInfo :: Uhyve { .. }) }
};
}
