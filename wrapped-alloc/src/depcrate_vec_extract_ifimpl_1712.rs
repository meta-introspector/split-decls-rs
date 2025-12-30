// Generated macro for impl_1712 (impl)
macro_rules! Depcrate_vec_extract_ifimpl_1712 {
() => {
// Module: crate::vec::extract_if
// Provides: {"impl_1712"}
// Dependencies: {}
impl < 'a , T , F , A : Allocator > ExtractIf < 'a , T , F , A > { pub (super) fn new < R : RangeBounds < usize > > (vec : & 'a mut Vec < T , A > , pred : F , range : R) -> Self { let old_len = vec . len () ; let Range { start , end } = slice :: range (range , .. old_len) ; unsafe { vec . set_len (0) ; } ExtractIf { vec , idx : start , del : 0 , end , old_len , pred } } # [doc = " Returns a reference to the underlying allocator."] # [unstable (feature = "allocator_api" , issue = "32838")] # [inline] pub fn allocator (& self) -> & A { self . vec . allocator () } }
};
}
