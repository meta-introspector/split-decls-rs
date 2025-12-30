// Generated macro for impl_1846 (impl)
macro_rules! Depcrate_vec_in_place_collectimpl_1846 {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"impl_1846"}
// Dependencies: {}
impl < T , I > SpecInPlaceCollect < T , I > for I where I : Iterator < Item = T > , { # [inline] default unsafe fn collect_in_place (& mut self , dst_buf : * mut T , end : * const T) -> usize { let sink = InPlaceDrop { inner : dst_buf , dst : dst_buf } ; let sink = self . try_fold :: < _ , _ , Result < _ , ! > > (sink , write_in_place_with_drop (end)) . into_ok () ; unsafe { ManuallyDrop :: new (sink) . dst . offset_from_unsigned (dst_buf) } } }
};
}
