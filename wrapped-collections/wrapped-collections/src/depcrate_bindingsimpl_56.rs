// Generated macro for impl_56 (impl)
macro_rules! Depcrate_bindingsimpl_56 {
() => {
// Module: crate::bindings
// Provides: {"impl_56"}
// Dependencies: {}
impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for & IMapView < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}
