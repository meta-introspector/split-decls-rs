// Generated macro for macro_158 (macro)
macro_rules! Depcrate_setup_configmacro_158 {
() => {
// Module: crate::setup_config
// Provides: {"macro_158"}
// Dependencies: {}
RIDL ! { # [uuid (0x6380bcff , 0x41d3 , 0x4b2e , 0x8b , 0x2e , 0xbf , 0x8a , 0x68 , 0x10 , 0xc8 , 0x48)] interface IEnumSetupInstances (IEnumSetupInstancesVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut ISetupInstance , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSetupInstances ,) -> HRESULT , } }
};
}
