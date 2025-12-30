// Generated macro for StockMapViewIterator (struct)
macro_rules! Depcrate_map_viewStockMapViewIterator {
() => {
// Module: crate::map_view
// Provides: {"StockMapViewIterator"}
// Dependencies: {}
# [implement (IIterator < IKeyValuePair < K , V >>)] struct StockMapViewIterator < 'a , K , V > where K : RuntimeType + 'static , V : RuntimeType + 'static , K :: Default : Clone + Ord , V :: Default : Clone , { _owner : ComObject < StockMapView < K , V > > , current : std :: sync :: RwLock < std :: collections :: btree_map :: Iter < 'a , K :: Default , V :: Default > > , }
};
}
