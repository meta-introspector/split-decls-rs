// Generated macro for impl_1842 (impl)
macro_rules! Depcrate_vec_in_place_collectimpl_1842 {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"impl_1842"}
// Dependencies: {}
impl < T , I > SpecFromIter < T , I > for Vec < T > where I : Iterator < Item = T > + InPlaceCollect , < I as SourceIter > :: Source : AsVecIntoIter , { # [track_caller] default fn from_iter (iterator : I) -> Self { let fun : fn (I) -> Vec < T > = const { if in_place_collectible :: < T , I :: Src > (I :: MERGE_BY , I :: EXPAND_BY) { from_iter_in_place } else { SpecFromIterNested :: < T , I > :: from_iter } } ; fun (iterator) } }
};
}
