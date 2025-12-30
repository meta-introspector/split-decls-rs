// Generated macro for impl_520 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_520 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_520"}
// Dependencies: {}
impl < K , V > CacheStorage for LruCacheImpl < K , V > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { type Key = K ; type Value = V ; # [inline] fn get (& mut self , key : & Self :: Key) -> Option < & Self :: Value > { self . 0 . get (key) } # [inline] fn insert (& mut self , key : Cow < '_ , Self :: Key > , val : Cow < '_ , Self :: Value >) { self . 0 . put (key . into_owned () , val . into_owned ()) ; } # [inline] fn remove (& mut self , key : & Self :: Key) { self . 0 . pop (key) ; } # [inline] fn clear (& mut self) { self . 0 . clear () ; } fn iter (& self) -> Box < dyn Iterator < Item = (& '_ Self :: Key , & '_ Self :: Value) > + '_ > { Box :: new (self . 0 . iter ()) } }
};
}
