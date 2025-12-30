// Generated macro for impl_123 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_123 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , K : 'a , V : 'a > StoreIterableMut < 'a , K , V > for ShortBoxSlice < (K , V) > { type KeyValueIterMut = core :: iter :: Map < core :: slice :: IterMut < 'a , (K , V) > , for < 'r > fn (& 'r mut (K , V)) -> (& 'r K , & 'r mut V) , > ; fn lm_iter_mut (& 'a mut self ,) -> < Self as litemap :: store :: StoreIterableMut < 'a , K , V > > :: KeyValueIterMut { self . iter_mut () . map (| elt | (& elt . 0 , & mut elt . 1)) } }
};
}
