// Generated macro for impl_233 (impl)
macro_rules! Depcrate_vec_set_len_on_dropimpl_233 {
() => {
// Module: crate::vec::set_len_on_drop
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'a > SetLenOnDrop < 'a > { # [inline (always)] pub (super) fn new (len : & 'a mut usize) -> Self { SetLenOnDrop { local_len : * len , len , } } # [inline (always)] pub (super) fn increment_len (& mut self , increment : usize) { self . local_len += increment ; } }
};
}
