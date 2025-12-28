macro_rules! InternMap {
    () => {
        pub type InternMap < T > = DashMap < Arc < T > , () , BuildHasherDefault < FxHasher > > ;
    };
}

InternMap!();