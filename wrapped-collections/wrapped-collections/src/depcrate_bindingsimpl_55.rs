// Generated macro for impl_55 (impl)
macro_rules! Depcrate_bindingsimpl_55 {
() => {
// Module: crate::bindings
// Provides: {"impl_55"}
// Dependencies: {}
impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for IMapView < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}
