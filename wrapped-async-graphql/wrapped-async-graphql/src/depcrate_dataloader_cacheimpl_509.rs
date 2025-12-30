// Generated macro for impl_509 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_509 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_509"}
// Dependencies: {}
impl < K , V > CacheStorage for NoCacheImpl < K , V > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { type Key = K ; type Value = V ; # [inline] fn get (& mut self , _key : & K) -> Option < & V > { None } # [inline] fn insert (& mut self , _key : Cow < '_ , Self :: Key > , _val : Cow < '_ , Self :: Value >) { } # [inline] fn remove (& mut self , _key : & K) { } # [inline] fn clear (& mut self) { } fn iter (& self) -> Box < dyn Iterator < Item = (& '_ Self :: Key , & '_ Self :: Value) > + '_ > { Box :: new (std :: iter :: empty ()) } }
};
}
