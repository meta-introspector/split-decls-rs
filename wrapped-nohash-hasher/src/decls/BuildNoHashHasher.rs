macro_rules! deps {
    () => {
        IntSet!();
        NoHashHasher!();
        IntMap!();
    };
}

macro_rules! BuildNoHashHasher {
    () => {
        deps!();
        # [doc = " An alias for `BuildHasherDefault` for use with `NoHashHasher`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " See also [`IntMap`] and [`IntSet`] for some easier usage examples."] # [doc = ""] # [doc = " ```"] # [doc = " use nohash_hasher::BuildNoHashHasher;"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut m: HashMap::<u8, char, BuildNoHashHasher<u8>> ="] # [doc = "     HashMap::with_capacity_and_hasher(2, BuildNoHashHasher::default());"] # [doc = ""] # [doc = " m.insert(0, 'a');"] # [doc = " m.insert(1, 'b');"] # [doc = ""] # [doc = " assert_eq!(Some(&'a'), m.get(&0));"] # [doc = " assert_eq!(Some(&'b'), m.get(&1));"] # [doc = " ```"] pub type BuildNoHashHasher < T > = BuildHasherDefault < NoHashHasher < T > > ;
    };
}

BuildNoHashHasher!()