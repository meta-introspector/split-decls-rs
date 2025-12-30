// Generated macro for impl_92 (impl)
macro_rules! Depcrate_store_slice_implimpl_92 {
() => {
// Module: crate::store::slice_impl
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > StoreIterable < 'a , K , V > for & 'a [(K , V)] { type KeyValueIter = core :: iter :: Map < core :: slice :: Iter < 'a , (K , V) > , MapF < K , V > > ; # [inline] fn lm_iter (& 'a self) -> Self :: KeyValueIter { self . iter () . map (map_f) } }
};
}
