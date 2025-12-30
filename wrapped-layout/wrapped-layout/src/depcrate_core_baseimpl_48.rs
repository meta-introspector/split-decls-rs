// Generated macro for impl_48 (impl)
macro_rules! Depcrate_core_baseimpl_48 {
() => {
// Module: crate::core::base
// Provides: {"impl_48"}
// Dependencies: {}
impl Orientation { pub fn is_top_to_bottom (& self) -> bool { if let Orientation :: TopToBottom = self { return true ; } false } pub fn is_left_right (& self) -> bool { if let Orientation :: TopToBottom = self { return false ; } true } pub fn flip (& self) -> Orientation { if let Orientation :: TopToBottom = self { return Orientation :: LeftToRight ; } Orientation :: TopToBottom } }
};
}
