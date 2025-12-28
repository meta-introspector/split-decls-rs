macro_rules! deps {
    () => {
        IKeyValuePair!();
        IMapView!();
        IIterable!();
    };
}

macro_rules! StockMapView {
    () => {
        deps!();
        # [implement (IMapView < K , V >, IIterable < IKeyValuePair < K , V >>)] struct StockMapView < K , V > where K : RuntimeType + 'static , V : RuntimeType + 'static , K :: Default : Clone + Ord , V :: Default : Clone , { map : std :: collections :: BTreeMap < K :: Default , V :: Default > , }
    };
}

StockMapView!();