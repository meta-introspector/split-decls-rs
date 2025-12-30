// Generated macro for impl_302 (impl)
macro_rules! Depcrate_streamimpl_302 {
() => {
// Module: crate::stream
// Provides: {"impl_302"}
// Dependencies: {}
impl < 'a , T > Positioned for & 'a [T] where T : Clone + PartialEq , { # [inline] fn position (& self) -> Self :: Position { PointerOffset :: new (self . as_ptr () as usize) } }
};
}
