// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bindingsimpl_30 {
() => {
// Module: crate::bindings
// Provides: {"impl_30"}
// Dependencies: {}
impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IKeyValuePair < K , V > { pub fn Key (& self) -> windows_core :: Result < K > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Key) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } pub fn Value (& self) -> windows_core :: Result < V > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Value) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
