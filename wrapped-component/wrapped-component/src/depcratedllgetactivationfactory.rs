// Generated macro for DllGetActivationFactory (function)
macro_rules! DepcrateDllGetActivationFactory {
() => {
// Module: crate
// Provides: {"DllGetActivationFactory"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn DllGetActivationFactory (name : Ref < HSTRING > , factory : OutRef < IActivationFactory > ,) -> HRESULT { if * name == "test_component.Class" { factory . write (Some (ClassFactory . into ())) . into () } else { _ = factory . write (None) ; CLASS_E_CLASSNOTAVAILABLE } }
};
}
