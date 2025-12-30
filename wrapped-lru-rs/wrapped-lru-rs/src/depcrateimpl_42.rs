// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { if self . len == 0 { return None ; } let key = unsafe { & (* (* self . end) . key . as_ptr ()) as & K } ; let val = unsafe { & (* (* self . end) . val . as_ptr ()) as & V } ; self . len -= 1 ; self . end = unsafe { (* self . end) . prev } ; Some ((key , val)) } }
};
}
