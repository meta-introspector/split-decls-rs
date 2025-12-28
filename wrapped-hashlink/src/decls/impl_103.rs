macro_rules! deps {
    () => {
        LinkedHashMap!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a LinkedHashMap < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
    };
}

impl_103!()