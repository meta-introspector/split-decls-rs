// Generated macro for impl_1055 (impl)
macro_rules! Depcrate_config_tree_keysimpl_1055 {
() => {
// Module: crate::config::tree::keys
// Provides: {"impl_1055"}
// Dependencies: {}
# [doc = " Init other validate implementations"] impl < T : Validate > Any < T > { # [doc = " Create a new instance from `name` and `section`"] pub const fn new_with_validate (name : & 'static str , section : & 'static dyn Section , validate : T) -> Self { Any { name , section , subsection_requirement : Some (SubSectionRequirement :: Never) , link : None , note : None , validate , } } }
};
}
