// Generated macro for impl_834 (impl)
macro_rules! Depcrate_collections_linked_listimpl_834 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_834"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < & 'a mut T > { if self . len == 0 { None } else { self . head . map (| node | unsafe { let node = & mut * node . as_ptr () ; self . len -= 1 ; self . head = node . next ; & mut node . element }) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } # [inline] fn last (mut self) -> Option < & 'a mut T > { self . next_back () } }
};
}
