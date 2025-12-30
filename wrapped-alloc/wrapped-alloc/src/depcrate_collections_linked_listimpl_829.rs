// Generated macro for impl_829 (impl)
macro_rules! Depcrate_collections_linked_listimpl_829 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_829"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { if self . len == 0 { None } else { self . head . map (| node | unsafe { let node = & * node . as_ptr () ; self . len -= 1 ; self . head = node . next ; & node . element }) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } # [inline] fn last (mut self) -> Option < & 'a T > { self . next_back () } }
};
}
