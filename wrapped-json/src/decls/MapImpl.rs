macro_rules! MapImpl {
    () => {
        # [cfg (feature = "preserve_order")] type MapImpl < K , V > = IndexMap < K , V > ;
    };
}

MapImpl!()