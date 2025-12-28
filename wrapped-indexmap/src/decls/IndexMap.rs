macro_rules! IndexMap {
    () => {
        # [cfg (not (feature = "std"))] pub struct IndexMap < K , V , S > { pub (crate) core : IndexMapCore < K , V > , hash_builder : S , }
    };
}

IndexMap!()