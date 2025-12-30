// Generated macro for macro_159 (macro)
macro_rules! Depcrate_setup_configmacro_159 {
() => {
// Module: crate::setup_config
// Provides: {"macro_159"}
// Dependencies: {}
RIDL ! { # [uuid (0x42843719 , 0xdb4c , 0x46c2 , 0x8e , 0x7c , 0x64 , 0xf1 , 0x81 , 0x6e , 0xfd , 0x5b)] interface ISetupConfiguration (ISetupConfigurationVtbl) : IUnknown (IUnknownVtbl) { fn EnumInstances (ppEnumInstances : * mut * mut IEnumSetupInstances ,) -> HRESULT , fn GetInstanceForCurrentProcess (ppInstance : * mut * mut ISetupInstance ,) -> HRESULT , fn GetInstanceForPath (wzPath : LPCWSTR , ppInstance : * mut * mut ISetupInstance ,) -> HRESULT , } }
};
}
