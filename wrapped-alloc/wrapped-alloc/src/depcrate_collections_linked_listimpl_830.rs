// Generated macro for impl_830 (impl)
macro_rules! Depcrate_collections_linked_listimpl_830 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_830"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a T > { if self . len == 0 { None } else { self . tail . map (| node | unsafe { let node = & * node . as_ptr () ; self . len -= 1 ; self . tail = node . prev ; & node . element }) } } }
};
}
