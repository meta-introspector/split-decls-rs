// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for IterMut < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a mut T > { if self . len == 0 { return None ; } self . len -= 1 ; let head = wrap_add (self . tail , self . len , self . ring . len ()) ; unsafe { let elem = self . ring . get_unchecked_mut (head) . assume_init_mut () ; Some (std :: mem :: transmute :: < & mut T , & 'a mut T > (elem)) } } }
};
}
