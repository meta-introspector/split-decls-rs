// Generated macro for StockMapView (struct)
macro_rules! Depcrate_map_viewStockMapView {
() => {
// Module: crate::map_view
// Provides: {"StockMapView"}
// Dependencies: {}
# [implement (IMapView < K , V >, IIterable < IKeyValuePair < K , V >>)] struct StockMapView < K , V > where K : RuntimeType + 'static , V : RuntimeType + 'static , K :: Default : Clone + Ord , V :: Default : Clone , { map : std :: collections :: BTreeMap < K :: Default , V :: Default > , }
};
}
