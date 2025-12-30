// Generated macro for impl_334 (impl)
macro_rules! Depcrate_object_tree_diffimpl_334 {
() => {
// Module: crate::object::tree::diff
// Provides: {"impl_334"}
// Dependencies: {}
impl Platform < '_ , '_ > { # [doc = " Adjust diff options with `change_opts`."] pub fn options (& mut self , change_opts : impl FnOnce (& mut crate :: diff :: Options)) -> & mut Self { change_opts (& mut self . options) ; self } }
};
}
