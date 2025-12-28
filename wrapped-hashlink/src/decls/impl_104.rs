macro_rules! deps {
    () => {
        IntoIter!();
        IterMut!();
        LinkedHashMap!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a mut LinkedHashMap < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
    };
}

impl_104!();