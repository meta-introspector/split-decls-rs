// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_config_tree_keysimpl_1079 {
() => {
// Module: crate::config::tree::keys
// Provides: {"impl_1079"}
// Dependencies: {}
impl String { # [doc = " Create a new instance."] pub const fn new_string (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: String) } }
};
}
