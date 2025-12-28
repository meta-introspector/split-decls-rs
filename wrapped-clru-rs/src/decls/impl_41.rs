macro_rules! deps {
    () => {
        CLruCache!();
        CLruCacheIterMut!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a mut CLruCache < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = CLruCacheIterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> CLruCacheIterMut < 'a , K , V > { self . iter_mut () } }
    };
}

impl_41!();