// Generated macro for StockKeyValuePair (struct)
macro_rules! Depcrate_map_viewStockKeyValuePair {
() => {
// Module: crate::map_view
// Provides: {"StockKeyValuePair"}
// Dependencies: {}
# [implement (IKeyValuePair < K , V >)] struct StockKeyValuePair < K , V > where K : RuntimeType + 'static , V : RuntimeType + 'static , K :: Default : Clone , V :: Default : Clone , { key : K :: Default , value : V :: Default , }
};
}
