macro_rules! deps {
    () => {
        LruCacheStorage!();
        CacheStorage!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        # [async_trait :: async_trait] impl CacheStorage for LruCacheStorage { async fn get (& self , key : String) -> Option < ExecutableDocument > { let mut cache = self . 0 . lock () . await ; cache . get (& key) . cloned () } async fn set (& self , key : String , query : ExecutableDocument) { let mut cache = self . 0 . lock () . await ; cache . put (key , query) ; } }
    };
}

impl_530!();