macro_rules! deps {
    () => {
        CLruCache!();
        WeightScale!();
        CLruCacheIter!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , K , V , S , W : WeightScale < K , V > > IntoIterator for & 'a CLruCache < K , V , S , W > { type Item = (& 'a K , & 'a V) ; type IntoIter = CLruCacheIter < 'a , K , V > ; # [inline] fn into_iter (self) -> CLruCacheIter < 'a , K , V > { self . iter () } }
    };
}

impl_36!()