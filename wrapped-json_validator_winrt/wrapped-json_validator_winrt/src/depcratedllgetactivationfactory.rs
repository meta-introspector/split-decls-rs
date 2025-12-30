// Generated macro for DllGetActivationFactory (function)
macro_rules! DepcrateDllGetActivationFactory {
() => {
// Module: crate
// Provides: {"DllGetActivationFactory"}
// Dependencies: {}
# [no_mangle] extern "system" fn DllGetActivationFactory (name : Ref < HSTRING > , factory : OutRef < IActivationFactory > ,) -> HRESULT { if * name == "Sample.JsonValidator" { factory . write (Some (JSON_VALIDATOR_FACTORY . to_interface ())) . into () } else { _ = factory . write (None) ; CLASS_E_CLASSNOTAVAILABLE } }
};
}
