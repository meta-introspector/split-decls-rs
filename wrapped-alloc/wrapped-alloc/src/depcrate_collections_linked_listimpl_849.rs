// Generated macro for impl_849 (impl)
macro_rules! Depcrate_collections_linked_listimpl_849 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_849"}
// Dependencies: {}
# [stable (feature = "extract_if" , since = "1.87.0")] impl < T , F , A : Allocator > Iterator for ExtractIf < '_ , T , F , A > where F : FnMut (& mut T) -> bool , { type Item = T ; fn next (& mut self) -> Option < T > { while let Some (mut node) = self . it { unsafe { self . it = node . as_ref () . next ; self . idx += 1 ; if (self . pred) (& mut node . as_mut () . element) { self . list . unlink_node (node) ; return Some (Box :: from_raw_in (node . as_ptr () , & self . list . alloc) . element) ; } } } None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . old_len - self . idx)) } }
};
}
