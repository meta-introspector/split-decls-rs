// Generated macro for impl_43 (impl)
macro_rules! Depcrate_bindingsimpl_43 {
() => {
// Module: crate::bindings
// Provides: {"impl_43"}
// Dependencies: {}
impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for & IMap < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}
