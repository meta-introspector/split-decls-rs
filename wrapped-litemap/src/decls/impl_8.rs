macro_rules! impl_8 {
    () => {
        # [cfg (feature = "alloc")] impl < K , V > LiteMap < K , V , Vec < (K , V) > > { # [doc = " Convert a [`LiteMap`] into a sorted `Vec<(K, V)>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] pub fn into_tuple_vec (self) -> Vec < (K , V) > { self . values } }
    };
}

impl_8!()