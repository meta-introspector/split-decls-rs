// Generated macro for impl_46 (impl)
macro_rules! Depcrate_core_baseimpl_46 {
() => {
// Module: crate::core::base
// Provides: {"impl_46"}
// Dependencies: {}
impl Direction { pub fn is_down (& self) -> bool { match self { Direction :: None | Direction :: Up => false , Direction :: Both | Direction :: Down => true , } } pub fn is_up (& self) -> bool { match self { Direction :: Both | Direction :: Up => true , Direction :: None | Direction :: Down => false , } } }
};
}
