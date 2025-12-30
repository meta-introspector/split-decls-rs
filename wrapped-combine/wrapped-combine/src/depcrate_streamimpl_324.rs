// Generated macro for impl_324 (impl)
macro_rules! Depcrate_streamimpl_324 {
() => {
// Module: crate::stream
// Provides: {"impl_324"}
// Dependencies: {}
impl < 'a , T > Positioned for SliceStream < 'a , T > where T : PartialEq + 'a , { # [inline] fn position (& self) -> Self :: Position { PointerOffset :: new (self . 0 . as_ptr () as usize) } }
};
}
