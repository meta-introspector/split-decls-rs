// Generated macro for impl_42 (impl)
macro_rules! Depcrate_bindingsimpl_42 {
() => {
// Module: crate::bindings
// Provides: {"impl_42"}
// Dependencies: {}
impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for IMap < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}
