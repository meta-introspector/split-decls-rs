// Generated macro for IJsonValidatorFactory_Impl (trait)
macro_rules! Depcrate_bindingsIJsonValidatorFactory_Impl {
() => {
// Module: crate::bindings
// Provides: {"IJsonValidatorFactory_Impl"}
// Dependencies: {}
pub trait IJsonValidatorFactory_Impl : windows_core :: IUnknownImpl { fn CreateInstance (& self , schema : & windows_core :: HSTRING) -> windows_core :: Result < JsonValidator > ; }
};
}
