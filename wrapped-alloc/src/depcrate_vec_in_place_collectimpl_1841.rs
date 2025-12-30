// Generated macro for impl_1841 (impl)
macro_rules! Depcrate_vec_in_place_collectimpl_1841 {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"impl_1841"}
// Dependencies: {}
impl < T > InPlaceCollect for T where T : SourceIter < Source : AsVecIntoIter > + InPlaceIterable , { type Src = < < T as SourceIter > :: Source as AsVecIntoIter > :: Item ; }
};
}
