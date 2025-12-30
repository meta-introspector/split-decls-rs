// Generated macro for impl_824 (impl)
macro_rules! Depcrate_config_snapshot_accessimpl_824 {
() => {
// Module: crate::config::snapshot::access
// Provides: {"impl_824"}
// Dependencies: {}
# [doc = " Utilities and additional access"] impl Snapshot < '_ > { # [doc = " Returns the underlying configuration implementation for a complete API, despite being a little less convenient."] # [doc = ""] # [doc = " It's expected that more functionality will move up depending on demand."] pub fn plumbing (& self) -> & gix_config :: File < 'static > { & self . repo . config . resolved } }
};
}
