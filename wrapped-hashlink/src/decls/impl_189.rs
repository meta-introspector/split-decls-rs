macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
        LruCache!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a LruCache < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
    };
}

impl_189!()