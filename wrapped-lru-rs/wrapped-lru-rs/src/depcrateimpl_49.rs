// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . len == 0 { return None ; } let key = unsafe { & mut (* (* self . ptr) . key . as_mut_ptr ()) as & mut K } ; let val = unsafe { & mut (* (* self . ptr) . val . as_mut_ptr ()) as & mut V } ; self . len -= 1 ; self . ptr = unsafe { (* self . ptr) . next } ; Some ((key , val)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } fn count (self) -> usize { self . len } }
};
}
