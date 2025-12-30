// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . len == 0 { return None ; } let key = unsafe { & mut (* (* self . end) . key . as_mut_ptr ()) as & mut K } ; let val = unsafe { & mut (* (* self . end) . val . as_mut_ptr ()) as & mut V } ; self . len -= 1 ; self . end = unsafe { (* self . end) . prev } ; Some ((key , val)) } }
};
}
