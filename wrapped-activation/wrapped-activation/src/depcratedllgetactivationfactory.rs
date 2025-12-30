// Generated macro for DllGetActivationFactory (function)
macro_rules! DepcrateDllGetActivationFactory {
() => {
// Module: crate
// Provides: {"DllGetActivationFactory"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn DllGetActivationFactory (name : Ref < HSTRING > , factory : OutRef < IActivationFactory > ,) -> HRESULT { if * name == "test_activation.One.Instance" { factory . write (Some (InstanceFactory . into ())) . into () } else if * name == "test_activation.One.Two.Three.Four.Static" { factory . write (Some (StaticFactory . into ())) . into () } else { _ = factory . write (None) ; CLASS_E_CLASSNOTAVAILABLE } }
};
}
