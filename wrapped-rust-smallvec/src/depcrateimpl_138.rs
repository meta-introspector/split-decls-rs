// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl < T , const N : usize > DoubleEndedIterator for IntoIter < T , N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { let mut end = self . end . value (Self :: is_zst ()) ; if self . begin == end { None } else { unsafe { let ptr = self . as_mut_ptr () ; let on_heap = self . end . on_heap (Self :: is_zst ()) ; end -= 1 ; self . end = TaggedLen :: new (end , on_heap , Self :: is_zst ()) ; let value = ptr . add (end) . read () ; Some (value) } } } }
};
}
