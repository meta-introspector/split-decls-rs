macro_rules! deps {
    () => {
        WeightScale!();
        CLruCache!();
        CLruCacheIter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < K , V , S , W : WeightScale < K , V > > CLruCache < K , V , S , W > { # [doc = " Returns an iterator visiting all entries in order."] # [doc = " The iterator element type is `(&'a K, &'a V)`."] pub fn iter (& self) -> CLruCacheIter < '_ , K , V > { CLruCacheIter { iter : self . storage . iter () , } } }
    };
}

impl_29!()