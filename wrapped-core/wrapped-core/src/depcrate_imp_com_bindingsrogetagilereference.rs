// Generated macro for RoGetAgileReference (function)
macro_rules! Depcrate_imp_com_bindingsRoGetAgileReference {
() => {
// Module: crate::imp::com_bindings
// Provides: {"RoGetAgileReference"}
// Dependencies: {}
# [inline] pub unsafe fn RoGetAgileReference < P2 > (options : AgileReferenceOptions , riid : * const windows_core :: GUID , punk : P2 ,) -> windows_core :: Result < IAgileReference > where P2 : windows_core :: Param < windows_core :: IUnknown > , { windows_core :: link ! ("combase.dll" "system" fn RoGetAgileReference (options : AgileReferenceOptions , riid : * const windows_core :: GUID , punk : * mut core :: ffi :: c_void , ppagilereference : * mut * mut core :: ffi :: c_void) -> windows_core :: HRESULT) ; unsafe { let mut result__ = core :: mem :: zeroed () ; RoGetAgileReference (options , riid , punk . param () . abi () , & mut result__) . and_then (| | windows_core :: Type :: from_abi (result__)) } }
};
}
