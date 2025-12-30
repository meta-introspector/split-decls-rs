// Generated macro for impl_8 (impl)
macro_rules! Depcrate_configimpl_8 {
() => {
// Module: crate::config
// Provides: {"impl_8"}
// Dependencies: {}
impl < K , V > CLruCacheConfig < K , V > { # [doc = " Creates a new configuration that will create an LRU cache"] # [doc = " that will hold at most `capacity` elements and default parameters."] pub fn new (capacity : NonZeroUsize) -> Self { Self { capacity , hash_builder : RandomState :: default () , reserve : None , scale : ZeroWeightScale , _marker : PhantomData , } } }
};
}
