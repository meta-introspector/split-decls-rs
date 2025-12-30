// Generated macro for impl_835 (impl)
macro_rules! Depcrate_collections_linked_listimpl_835 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_835"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > DoubleEndedIterator for IterMut < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a mut T > { if self . len == 0 { None } else { self . tail . map (| node | unsafe { let node = & mut * node . as_ptr () ; self . len -= 1 ; self . tail = node . prev ; & mut node . element }) } } }
};
}
