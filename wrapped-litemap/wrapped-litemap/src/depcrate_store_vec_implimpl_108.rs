// Generated macro for impl_108 (impl)
macro_rules! Depcrate_store_vec_implimpl_108 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > StoreIterableMut < 'a , K , V > for Vec < (K , V) > { type KeyValueIterMut = core :: iter :: Map < core :: slice :: IterMut < 'a , (K , V) > , MapFMut < K , V > > ; # [inline] fn lm_iter_mut (& 'a mut self) -> Self :: KeyValueIterMut { self . as_mut_slice () . iter_mut () . map (map_f_mut) } }
};
}
