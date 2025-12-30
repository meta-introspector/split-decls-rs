// Generated macro for impl_107 (impl)
macro_rules! Depcrate_store_vec_implimpl_107 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > StoreIterable < 'a , K , V > for Vec < (K , V) > { type KeyValueIter = core :: iter :: Map < core :: slice :: Iter < 'a , (K , V) > , MapF < K , V > > ; # [inline] fn lm_iter (& 'a self) -> Self :: KeyValueIter { self . as_slice () . iter () . map (map_f) } }
};
}
