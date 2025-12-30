// Generated macro for impl_533 (impl)
macro_rules! Depcrate_dataloaderimpl_533 {
() => {
// Module: crate::dataloader
// Provides: {"impl_533"}
// Dependencies: {}
impl < K : Send + Sync + Hash + Eq + Clone + 'static , T : Loader < K > > Requests < K , T > { fn new < C : CacheFactory > (cache_factory : & C) -> Self { Self { keys : Default :: default () , pending : Vec :: new () , cache_storage : cache_factory . create :: < K , T :: Value > () , disable_cache : false , } } fn take (& mut self) -> KeysAndSender < K , T > { (std :: mem :: take (& mut self . keys) , std :: mem :: take (& mut self . pending) ,) } }
};
}
