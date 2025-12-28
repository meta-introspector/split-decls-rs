macro_rules! deps {
    () => {
        StoreIterable!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a , S > LiteMap < K , V , S > where S : StoreIterable < 'a , K , V > , { # [doc = " Produce an ordered iterator over key-value pairs"] pub fn iter (& 'a self) -> impl DoubleEndedIterator < Item = (& 'a K , & 'a V) > { self . values . lm_iter () } # [doc = " Produce an ordered iterator over keys"] # [deprecated = "use keys() instead"] pub fn iter_keys (& 'a self) -> impl DoubleEndedIterator < Item = & 'a K > { self . values . lm_iter () . map (| val | val . 0) } # [doc = " Produce an iterator over values, ordered by their keys"] # [deprecated = "use values() instead"] pub fn iter_values (& 'a self) -> impl DoubleEndedIterator < Item = & 'a V > { self . values . lm_iter () . map (| val | val . 1) } # [doc = " Produce an ordered iterator over keys"] pub fn keys (& 'a self) -> impl DoubleEndedIterator < Item = & 'a K > { self . values . lm_iter () . map (| val | val . 0) } # [doc = " Produce an iterator over values, ordered by their keys"] pub fn values (& 'a self) -> impl DoubleEndedIterator < Item = & 'a V > { self . values . lm_iter () . map (| val | val . 1) } }
    };
}

impl_21!();