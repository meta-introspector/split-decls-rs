// Generated macro for impl_116 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_116 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_116"}
// Dependencies: {}
impl < K , V > StoreSlice < K , V > for ShortBoxSlice < (K , V) > { type Slice = [(K , V)] ; # [inline] fn lm_get_range (& self , range : core :: ops :: Range < usize >) -> Option < & Self :: Slice > { self . get (range) } }
};
}
