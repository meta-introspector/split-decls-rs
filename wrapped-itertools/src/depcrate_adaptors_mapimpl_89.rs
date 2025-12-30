// Generated macro for impl_89 (impl)
macro_rules! Depcrate_adaptors_mapimpl_89 {
() => {
// Module: crate::adaptors::map
// Provides: {"impl_89"}
// Dependencies: {}
impl < T : Into < U > , U > MapSpecialCaseFn < T > for MapSpecialCaseFnInto < U > { type Out = U ; fn call (& mut self , t : T) -> Self :: Out { t . into () } }
};
}
