// Generated macro for TreeFocusMut (struct)
macro_rules! Depcrate_vector_focusTreeFocusMut {
() => {
// Module: crate::vector::focus
// Provides: {"TreeFocusMut"}
// Dependencies: {}
pub struct TreeFocusMut < 'a , A > { tree : Lock < & 'a mut Rrb < A > > , view : Range < usize > , middle_range : Range < usize > , target_range : Range < usize > , target_ptr : AtomicPtr < Chunk < A > > , }
};
}
