// Generated macro for impl_515 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_515 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_515"}
// Dependencies: {}
impl < K , V , S > CacheStorage for HashMapCacheImpl < K , V , S > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , S : Send + Sync + BuildHasher + 'static , { type Key = K ; type Value = V ; # [inline] fn get (& mut self , key : & Self :: Key) -> Option < & Self :: Value > { self . 0 . get (key) } # [inline] fn insert (& mut self , key : Cow < '_ , Self :: Key > , val : Cow < '_ , Self :: Value >) { self . 0 . insert (key . into_owned () , val . into_owned ()) ; } # [inline] fn remove (& mut self , key : & Self :: Key) { self . 0 . remove (key) ; } # [inline] fn clear (& mut self) { self . 0 . clear () ; } fn iter (& self) -> Box < dyn Iterator < Item = (& '_ Self :: Key , & '_ Self :: Value) > + '_ > { Box :: new (self . 0 . iter ()) } }
};
}
