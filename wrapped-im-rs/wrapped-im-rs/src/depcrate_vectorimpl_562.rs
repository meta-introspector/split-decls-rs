// Generated macro for impl_562 (impl)
macro_rules! Depcrate_vectorimpl_562 {
() => {
// Module: crate::vector
// Provides: {"impl_562"}
// Dependencies: {}
impl < 'a , A : Clone > DoubleEndedIterator for Chunks < 'a , A > { # [doc = " Remove and return an element from the back of the iterator."] # [doc = ""] # [doc = " Time: O(1)*"] fn next_back (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } self . back_index -= 1 ; # [allow (unsafe_code)] let focus : & 'a mut Focus < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let (range , value) = focus . chunk_at (self . back_index) ; self . back_index = range . start ; Some (value) } }
};
}
