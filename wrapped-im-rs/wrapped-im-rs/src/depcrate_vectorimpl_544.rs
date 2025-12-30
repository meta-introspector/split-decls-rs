// Generated macro for impl_544 (impl)
macro_rules! Depcrate_vectorimpl_544 {
() => {
// Module: crate::vector
// Provides: {"impl_544"}
// Dependencies: {}
impl < 'a , A : Clone > DoubleEndedIterator for Iter < 'a , A > { # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next_back (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } self . back_index -= 1 ; # [allow (unsafe_code)] let focus : & 'a mut Focus < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; focus . get (self . back_index) } }
};
}
