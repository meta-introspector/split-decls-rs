macro_rules! deps {
    () => {
        CacheStorage!();
        NoCacheImpl!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < K , V > CacheStorage for NoCacheImpl < K , V > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { type Key = K ; type Value = V ; # [inline] fn get (& mut self , _key : & K) -> Option < & V > { None } # [inline] fn insert (& mut self , _key : Cow < '_ , Self :: Key > , _val : Cow < '_ , Self :: Value >) { } # [inline] fn remove (& mut self , _key : & K) { } # [inline] fn clear (& mut self) { } fn iter (& self) -> Box < dyn Iterator < Item = (& '_ Self :: Key , & '_ Self :: Value) > + '_ > { Box :: new (std :: iter :: empty ()) } }
    };
}

impl_365!();