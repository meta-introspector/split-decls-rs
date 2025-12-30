// Generated macro for impl_1054 (impl)
macro_rules! Depcrate_config_tree_keysimpl_1054 {
() => {
// Module: crate::config::tree::keys
// Provides: {"impl_1054"}
// Dependencies: {}
# [doc = " Init"] impl Any < validate :: All > { # [doc = " Create a new instance from `name` and `section`"] pub const fn new (name : & 'static str , section : & 'static dyn Section) -> Self { Any :: new_with_validate (name , section , validate :: All) } }
};
}
