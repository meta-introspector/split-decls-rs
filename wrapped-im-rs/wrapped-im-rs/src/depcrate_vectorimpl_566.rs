// Generated macro for impl_566 (impl)
macro_rules! Depcrate_vectorimpl_566 {
() => {
// Module: crate::vector
// Provides: {"impl_566"}
// Dependencies: {}
impl < 'a , A : Clone > Iterator for ChunksMut < 'a , A > { type Item = & 'a mut [A] ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } # [allow (unsafe_code)] let focus : & 'a mut FocusMut < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let (range , value) = focus . chunk_at (self . front_index) ; self . front_index = range . end ; Some (value) } }
};
}
