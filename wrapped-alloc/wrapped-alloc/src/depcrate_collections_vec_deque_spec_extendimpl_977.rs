// Generated macro for impl_977 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_extendimpl_977 {
() => {
// Module: crate::collections::vec_deque::spec_extend
// Provides: {"impl_977"}
// Dependencies: {}
impl < T , I , A : Allocator > SpecExtend < T , I > for VecDeque < T , A > where I : TrustedLen < Item = T > , { # [track_caller] default fn spec_extend (& mut self , iter : I) { let (low , high) = iter . size_hint () ; if let Some (additional) = high { debug_assert_eq ! (low , additional , "TrustedLen iterator's size hint is not exact: {:?}" , (low , high)) ; self . reserve (additional) ; let written = unsafe { self . write_iter_wrapping (self . to_physical_idx (self . len) , iter , additional) } ; debug_assert_eq ! (additional , written , "The number of items written to VecDeque doesn't match the TrustedLen size hint") ; } else { panic ! ("capacity overflow") ; } } }
};
}
