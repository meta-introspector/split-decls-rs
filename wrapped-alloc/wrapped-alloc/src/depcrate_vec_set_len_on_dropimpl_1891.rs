// Generated macro for impl_1891 (impl)
macro_rules! Depcrate_vec_set_len_on_dropimpl_1891 {
() => {
// Module: crate::vec::set_len_on_drop
// Provides: {"impl_1891"}
// Dependencies: {}
impl < 'a > SetLenOnDrop < 'a > { # [inline] pub (super) fn new (len : & 'a mut usize) -> Self { SetLenOnDrop { local_len : * len , len } } # [inline] pub (super) fn increment_len (& mut self , increment : usize) { self . local_len += increment ; } # [inline] pub (super) fn current_len (& self) -> usize { self . local_len } }
};
}
