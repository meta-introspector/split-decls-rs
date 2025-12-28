macro_rules! deps {
    () => {
        StoreBulkMut!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < K , V , S > Extend < (K , V) > for LiteMap < K , V , S > where K : Ord , S : StoreBulkMut < K , V > , { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . values . lm_extend (iter) } }
    };
}

impl_54!();