// Generated macro for IClosable_Vtbl (struct)
macro_rules! Depcrate_class_with_handlerIClosable_Vtbl {
() => {
// Module: crate::class_with_handler
// Provides: {"IClosable_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IClosable_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Close : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
