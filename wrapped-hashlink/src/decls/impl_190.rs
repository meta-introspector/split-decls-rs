macro_rules! deps {
    () => {
        LruCache!();
        IntoIter!();
        IterMut!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a mut LruCache < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
    };
}

impl_190!();