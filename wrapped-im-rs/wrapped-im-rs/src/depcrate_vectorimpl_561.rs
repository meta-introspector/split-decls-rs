// Generated macro for impl_561 (impl)
macro_rules! Depcrate_vectorimpl_561 {
() => {
// Module: crate::vector
// Provides: {"impl_561"}
// Dependencies: {}
impl < 'a , A : Clone > Iterator for Chunks < 'a , A > { type Item = & 'a [A] ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { if self . front_index >= self . back_index { return None ; } # [allow (unsafe_code)] let focus : & 'a mut Focus < 'a , A > = unsafe { & mut * (& mut self . focus as * mut _) } ; let (range , value) = focus . chunk_at (self . front_index) ; self . front_index = range . end ; Some (value) } }
};
}
