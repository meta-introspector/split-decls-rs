// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < & 'a mut T > { if self . len == 0 { return None ; } let tail = self . tail ; self . tail = wrap_add (self . tail , 1 , self . ring . len ()) ; self . len -= 1 ; unsafe { let elem = self . ring . get_unchecked_mut (tail) . assume_init_mut () ; Some (std :: mem :: transmute :: < & mut T , & 'a mut T > (elem)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
