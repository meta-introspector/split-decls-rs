// Generated macro for DllGetActivationFactory (function)
macro_rules! DepcrateDllGetActivationFactory {
() => {
// Module: crate
// Provides: {"DllGetActivationFactory"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn DllGetActivationFactory (name : Ref < HSTRING > , factory : OutRef < IActivationFactory > ,) -> HRESULT { if * name == "test_overloads.A" { factory . write (Some (FA . into ())) . into () } else if * name == "test_overloads.B" { factory . write (Some (FB . into ())) . into () } else if * name == "test_overloads.C" { factory . write (Some (FC . into ())) . into () } else if * name == "test_overloads.D" { factory . write (Some (FD . into ())) . into () } else if * name == "test_overloads.E" { factory . write (Some (FE . into ())) . into () } else { _ = factory . write (None) ; CLASS_E_CLASSNOTAVAILABLE } }
};
}
