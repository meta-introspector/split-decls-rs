// Generated macro for impl_156 (impl)
macro_rules! Depcrateimpl_156 {
() => {
// Module: crate
// Provides: {"impl_156"}
// Dependencies: {}
impl < T : Clone , const N : usize > From < & [T] > for SmallVec < T , N > { # [inline] fn from (slice : & [T]) -> Self { if slice . len () > Self :: inline_size () { Self :: from_vec (Vec :: from (slice)) } else { unsafe { # [cfg (feature = "specialization")] { < Self as spec_traits :: SpecFromSlice < T > > :: spec_from (slice) } # [cfg (not (feature = "specialization"))] { Self :: from_slice_fallback (slice) } } } } }
};
}
