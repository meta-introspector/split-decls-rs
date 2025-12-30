// Generated macro for impl_549 (impl)
macro_rules! Depcrate_vectorimpl_549 {
() => {
// Module: crate::vector
// Provides: {"impl_549"}
// Dependencies: {}
impl < 'a , A > Iterator for IterMut < 'a , A > where A : 'a + Clone , { type Item = & 'a mut A ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } # [allow (unsafe_code)] let focus : & 'a mut FocusMut < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let value = focus . get_mut (self . front_index) ; self . front_index += 1 ; value } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . back_index - self . front_index ; (remaining , Some (remaining)) } }
};
}
