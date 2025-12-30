// Generated macro for impl_28 (impl)
macro_rules! Depcrate_tree_rootimpl_28 {
() => {
// Module: crate::tree::root
// Provides: {"impl_28"}
// Dependencies: {}
impl crate :: WeakRoot for Weak < Root > { type Root = Arc < Root > ; fn upgrade (& self) -> Option < Self :: Root > { Weak :: upgrade (self) } }
};
}
