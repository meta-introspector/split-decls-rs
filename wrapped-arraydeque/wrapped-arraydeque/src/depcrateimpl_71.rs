// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { if self . len == 0 { return None ; } let tail = self . tail ; self . tail = wrap_add (self . tail , 1 , self . ring . len ()) ; self . len -= 1 ; unsafe { Some (self . ring . get_unchecked (tail) . assume_init_ref ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
