// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a T > { if self . len == 0 { return None ; } self . len -= 1 ; let head = wrap_add (self . tail , self . len , self . ring . len ()) ; unsafe { Some (self . ring . get_unchecked (head) . assume_init_ref ()) } } }
};
}
