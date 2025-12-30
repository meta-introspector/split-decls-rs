// Generated macro for impl_107 (impl)
macro_rules! Depcrate_map_viewimpl_107 {
() => {
// Module: crate::map_view
// Provides: {"impl_107"}
// Dependencies: {}
impl < K , V > From < std :: collections :: BTreeMap < K :: Default , V :: Default > > for IMapView < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn from (map : std :: collections :: BTreeMap < K :: Default , V :: Default >) -> Self { StockMapView { map } . into () } }
};
}
