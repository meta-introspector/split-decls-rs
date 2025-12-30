// Generated macro for impl_9 (impl)
macro_rules! Depcrate_bufferimpl_9 {
() => {
// Module: crate::buffer
// Provides: {"impl_9"}
// Dependencies: {}
impl ConsumeBuffer { pub fn from_vec (inner : Vec < u8 >) -> Self { ConsumeBuffer { inner , head : 0 } } pub fn into_vec (self) -> Vec < u8 > { let mut inner = self . inner ; inner . drain (0 .. self . head) ; inner } pub fn pop_front (& mut self , count : usize) { assert ! (self . head + count <= self . inner . len ()) ; self . head += count ; } pub fn expand (& mut self , count : usize) { self . inner . reserve_exact (count) ; unsafe { self . inner . set_len (count) } ; } pub fn truncate (& mut self , count : usize) { self . inner . truncate (self . head + count) ; } pub fn add_prefix (& mut self , prefix : & [u8]) -> bool { if self . head < prefix . len () { return false ; } self . head -= prefix . len () ; self . inner [self . head .. self . head + prefix . len ()] . copy_from_slice (prefix) ; true } }
};
}
