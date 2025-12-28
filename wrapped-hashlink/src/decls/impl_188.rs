macro_rules! deps {
    () => {
        IntoIter!();
        LruCache!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < K , V , S > IntoIterator for LruCache < K , V , S > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (self) -> IntoIter < K , V > { self . map . into_iter () } }
    };
}

impl_188!()