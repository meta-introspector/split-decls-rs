macro_rules! deps {
    () => {
        BuildNoHashHasher!();
        IsEnabled!();
        NoHashHasher!();
    };
}

macro_rules! IntSet {
    () => {
        deps!();
        # [doc = " A `HashSet` of integers, using `NoHashHasher` to perform no hashing at all."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " See [`IsEnabled`] for use with custom types."] # [doc = ""] # [doc = " ```"] # [doc = " use nohash_hasher::IntSet;"] # [doc = ""] # [doc = " let mut m = IntSet::default();"] # [doc = ""] # [doc = " m.insert(0u32);"] # [doc = " m.insert(1u32);"] # [doc = ""] # [doc = " assert!(m.contains(&0));"] # [doc = " assert!(m.contains(&1));"] # [doc = " ```"] # [cfg (feature = "std")] pub type IntSet < T > = std :: collections :: HashSet < T , BuildNoHashHasher < T > > ;
    };
}

IntSet!();