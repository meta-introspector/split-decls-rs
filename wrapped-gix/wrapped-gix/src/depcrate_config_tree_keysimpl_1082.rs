// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_config_tree_keysimpl_1082 {
() => {
// Module: crate::config::tree::keys
// Provides: {"impl_1082"}
// Dependencies: {}
impl Path { # [doc = " Create a new instance."] pub const fn new_path (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Path) } }
};
}
