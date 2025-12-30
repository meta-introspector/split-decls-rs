// Generated macro for impl_1847 (impl)
macro_rules! Depcrate_vec_in_place_collectimpl_1847 {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"impl_1847"}
// Dependencies: {}
impl < T , I > SpecInPlaceCollect < T , I > for I where I : Iterator < Item = T > + TrustedRandomAccessNoCoerce , { # [inline] unsafe fn collect_in_place (& mut self , dst_buf : * mut T , end : * const T) -> usize { let len = self . size () ; let mut drop_guard = InPlaceDrop { inner : dst_buf , dst : dst_buf } ; for i in 0 .. len { unsafe { let dst = dst_buf . add (i) ; debug_assert ! (dst as * const _ <= end , "InPlaceIterable contract violation") ; ptr :: write (dst , self . __iterator_get_unchecked (i)) ; drop_guard . dst = dst . add (1) ; } } mem :: forget (drop_guard) ; len } }
};
}
