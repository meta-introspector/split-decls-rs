macro_rules! HashMap {
    () => {
        # [cfg (not (feature = "hp-hashmap"))] type HashMap < K , V > = hashmap :: Concurrent < K , V > ;
    };
}

HashMap!()