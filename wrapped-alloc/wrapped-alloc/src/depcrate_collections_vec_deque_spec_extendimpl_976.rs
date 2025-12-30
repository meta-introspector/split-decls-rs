// Generated macro for impl_976 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_extendimpl_976 {
() => {
// Module: crate::collections::vec_deque::spec_extend
// Provides: {"impl_976"}
// Dependencies: {}
impl < T , I , A : Allocator > SpecExtend < T , I > for VecDeque < T , A > where I : Iterator < Item = T > , { # [track_caller] default fn spec_extend (& mut self , mut iter : I) { while let Some (element) = iter . next () { let (lower , _) = iter . size_hint () ; self . reserve (lower . saturating_add (1)) ; unsafe { self . push_unchecked (element) } ; while self . len < self . capacity () { let Some (element) = iter . next () else { return ; } ; unsafe { self . push_unchecked (element) } ; } } } }
};
}
