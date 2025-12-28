macro_rules! deps {
    () => {
        IKeyValuePair!();
    };
}

macro_rules! StockKeyValuePair {
    () => {
        deps!();
        # [implement (IKeyValuePair < K , V >)] struct StockKeyValuePair < K , V > where K : RuntimeType + 'static , V : RuntimeType + 'static , K :: Default : Clone , V :: Default : Clone , { key : K :: Default , value : V :: Default , }
    };
}

StockKeyValuePair!();