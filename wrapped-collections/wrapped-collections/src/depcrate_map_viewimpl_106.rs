// Generated macro for impl_106 (impl)
macro_rules! Depcrate_map_viewimpl_106 {
() => {
// Module: crate::map_view
// Provides: {"impl_106"}
// Dependencies: {}
impl < K , V > IKeyValuePair_Impl < K , V > for StockKeyValuePair_Impl < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone , V :: Default : Clone , { fn Key (& self) -> Result < K > { K :: from_default (& self . key) } fn Value (& self) -> Result < V > { V :: from_default (& self . value) } }
};
}
