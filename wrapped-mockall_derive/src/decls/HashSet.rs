macro_rules! HashSet {
    () => {
        type HashSet < K > = std :: collections :: HashSet < K , BuildHasherDefault < std :: collections :: hash_map :: DefaultHasher > > ;
    };
}

HashSet!();