macro_rules! HashMap {
    () => {
        type HashMap < K , V > = std :: collections :: HashMap < K , V , BuildHasherDefault < std :: collections :: hash_map :: DefaultHasher > > ;
    };
}

HashMap!();