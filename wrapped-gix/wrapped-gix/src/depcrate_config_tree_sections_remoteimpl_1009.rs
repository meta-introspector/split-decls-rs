// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_config_tree_sections_remoteimpl_1009 {
() => {
// Module: crate::config::tree::sections::remote
// Provides: {"impl_1009"}
// Dependencies: {}
impl Section for Remote { fn name (& self) -> & str { "remote" } fn keys (& self) -> & [& dyn Key] { & [& Self :: PUSH_DEFAULT , & Self :: TAG_OPT , & Self :: URL , & Self :: PUSH_URL , & Self :: FETCH , & Self :: PUSH , & Self :: PROXY , & Self :: PROXY_AUTH_METHOD ,] } }
};
}
