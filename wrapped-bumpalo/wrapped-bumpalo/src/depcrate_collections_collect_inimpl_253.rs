// Generated macro for impl_253 (impl)
macro_rules! Depcrate_collections_collect_inimpl_253 {
() => {
// Module: crate::collections::collect_in
// Provides: {"impl_253"}
// Dependencies: {}
impl < T , V : FromIteratorIn < T > > FromIteratorIn < Option < T > > for Option < V > { type Alloc = V :: Alloc ; fn from_iter_in < I > (iter : I , alloc : Self :: Alloc) -> Self where I : IntoIterator < Item = Option < T > > , { iter . into_iter () . map (| x | x . ok_or (())) . collect_in :: < Result < _ , _ > > (alloc) . ok () } }
};
}
