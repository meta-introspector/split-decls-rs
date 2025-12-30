// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { if self . len == 0 { return None ; } let key = unsafe { & (* (* self . ptr) . key . as_ptr ()) as & K } ; let val = unsafe { & (* (* self . ptr) . val . as_ptr ()) as & V } ; self . len -= 1 ; self . ptr = unsafe { (* self . ptr) . next } ; Some ((key , val)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } fn count (self) -> usize { self . len } }
};
}
