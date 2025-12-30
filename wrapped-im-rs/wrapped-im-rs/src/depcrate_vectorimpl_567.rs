// Generated macro for impl_567 (impl)
macro_rules! Depcrate_vectorimpl_567 {
() => {
// Module: crate::vector
// Provides: {"impl_567"}
// Dependencies: {}
impl < 'a , A : Clone > DoubleEndedIterator for ChunksMut < 'a , A > { # [doc = " Remove and return an element from the back of the iterator."] # [doc = ""] # [doc = " Time: O(1)*"] fn next_back (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } self . back_index -= 1 ; # [allow (unsafe_code)] let focus : & 'a mut FocusMut < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let (range , value) = focus . chunk_at (self . back_index) ; self . back_index = range . start ; Some (value) } }
};
}
