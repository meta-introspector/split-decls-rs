macro_rules! deps {
    () => {
        NoHashHasher!();
        BuildNoHashHasher!();
        IsEnabled!();
    };
}

macro_rules! IntMap {
    () => {
        deps!();
        # [doc = " A `HashMap` with an integer domain, using `NoHashHasher` to perform no hashing at all."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " See [`IsEnabled`] for use with custom types."] # [doc = ""] # [doc = " ```"] # [doc = " use nohash_hasher::IntMap;"] # [doc = ""] # [doc = " let mut m: IntMap<u32, bool> = IntMap::default();"] # [doc = ""] # [doc = " m.insert(0, false);"] # [doc = " m.insert(1, true);"] # [doc = ""] # [doc = " assert!(m.contains_key(&0));"] # [doc = " assert!(m.contains_key(&1));"] # [doc = " ```"] # [cfg (feature = "std")] pub type IntMap < K , V > = std :: collections :: HashMap < K , V , BuildNoHashHasher < K > > ;
    };
}

IntMap!();