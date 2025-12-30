// Generated macro for impl_94 (impl)
macro_rules! Depcrate_collections_vecimpl_94 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > SetLenOnDrop < 'a > { # [inline] fn new (len : & 'a mut usize) -> Self { SetLenOnDrop { local_len : * len , len , } } # [inline] fn increment_len (& mut self , increment : usize) { self . local_len += increment ; } # [inline] fn decrement_len (& mut self , decrement : usize) { self . local_len -= decrement ; } }
};
}
