// Generated macro for impl_121 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_121 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > StoreIterable < 'a , K , V > for ShortBoxSlice < (K , V) > { type KeyValueIter = core :: iter :: Map < core :: slice :: Iter < 'a , (K , V) > , for < 'r > fn (& 'r (K , V)) -> (& 'r K , & 'r V) > ; fn lm_iter (& 'a self) -> Self :: KeyValueIter { self . iter () . map (| elt | (& elt . 0 , & elt . 1)) } }
};
}
