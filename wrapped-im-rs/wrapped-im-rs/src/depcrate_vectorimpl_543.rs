// Generated macro for impl_543 (impl)
macro_rules! Depcrate_vectorimpl_543 {
() => {
// Module: crate::vector
// Provides: {"impl_543"}
// Dependencies: {}
impl < 'a , A : Clone > Iterator for Iter < 'a , A > { type Item = & 'a A ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } # [allow (unsafe_code)] let focus : & 'a mut Focus < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let value = focus . get (self . front_index) ; self . front_index += 1 ; value } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . back_index - self . front_index ; (remaining , Some (remaining)) } }
};
}
