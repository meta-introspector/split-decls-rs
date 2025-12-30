// Generated macro for impl_49 (impl)
macro_rules! Depcrate_impls_vec_dequeimpl_49 {
() => {
// Module: crate::impls::vec_deque
// Provides: {"impl_49"}
// Dependencies: {}
# [doc = " BufRead is implemented for `VecDeque<u8>` by reading bytes from the front of the `VecDeque`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl BufRead for VecDeque < u8 > { # [doc = " Returns the contents of the \"front\" slice as returned by"] # [doc = " [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are"] # [doc = " discontiguous, multiple calls to `fill_buf` will be needed to read the entire content."] # [inline] fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { let (front , _) = self . as_slices () ; Ok (front) } # [inline] fn consume (& mut self , amt : usize) { self . drain (.. amt) ; } }
};
}
