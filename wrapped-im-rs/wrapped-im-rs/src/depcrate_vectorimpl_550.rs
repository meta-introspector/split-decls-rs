// Generated macro for impl_550 (impl)
macro_rules! Depcrate_vectorimpl_550 {
() => {
// Module: crate::vector
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'a , A > DoubleEndedIterator for IterMut < 'a , A > where A : 'a + Clone , { # [doc = " Remove and return an element from the back of the iterator."] # [doc = ""] # [doc = " Time: O(1)*"] fn next_back (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } self . back_index -= 1 ; # [allow (unsafe_code)] let focus : & 'a mut FocusMut < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; focus . get_mut (self . back_index) } }
};
}
