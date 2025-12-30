// Generated macro for IKeyValuePair_Impl (trait)
macro_rules! Depcrate_bindingsIKeyValuePair_Impl {
() => {
// Module: crate::bindings
// Provides: {"IKeyValuePair_Impl"}
// Dependencies: {}
pub trait IKeyValuePair_Impl < K , V > : windows_core :: IUnknownImpl where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { fn Key (& self) -> windows_core :: Result < K > ; fn Value (& self) -> windows_core :: Result < V > ; }
};
}
