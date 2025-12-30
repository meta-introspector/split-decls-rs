// Generated macro for impl_1904 (impl)
macro_rules! Depcrate_vec_in_place_dropimpl_1904 {
() => {
// Module: crate::vec::in_place_drop
// Provides: {"impl_1904"}
// Dependencies: {}
impl < Src , Dest > Drop for InPlaceDstDataSrcBufDrop < Src , Dest > { # [inline] fn drop (& mut self) { unsafe { let _drop_allocation = RawVec :: < Src > :: from_nonnull_in (self . ptr . cast :: < Src > () , self . src_cap , Global) ; drop_in_place (core :: ptr :: slice_from_raw_parts_mut :: < Dest > (self . ptr . as_ptr () , self . len)) ; } ; } }
};
}
