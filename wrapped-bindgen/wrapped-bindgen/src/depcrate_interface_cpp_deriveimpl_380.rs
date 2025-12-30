// Generated macro for impl_380 (impl)
macro_rules! Depcrate_interface_cpp_deriveimpl_380 {
() => {
// Module: crate::interface_cpp_derive
// Provides: {"impl_380"}
// Dependencies: {}
impl IPersistFile { pub unsafe fn IsDirty (& self) -> windows_core :: HRESULT { unsafe { (windows_core :: Interface :: vtable (self) . IsDirty) (windows_core :: Interface :: as_raw (self)) } } pub unsafe fn Save < P0 > (& self , pszfilename : P0 , fremember : bool) -> windows_core :: Result < () > where P0 : windows_core :: Param < windows_core :: PCWSTR > , { unsafe { (windows_core :: Interface :: vtable (self) . Save) (windows_core :: Interface :: as_raw (self) , pszfilename . param () . abi () , fremember . into () ,) . ok () } } pub unsafe fn SaveCompleted < P0 > (& self , pszfilename : P0) -> windows_core :: Result < () > where P0 : windows_core :: Param < windows_core :: PCWSTR > , { unsafe { (windows_core :: Interface :: vtable (self) . SaveCompleted) (windows_core :: Interface :: as_raw (self) , pszfilename . param () . abi () ,) . ok () } } pub unsafe fn GetCurFile (& self) -> windows_core :: Result < windows_core :: PWSTR > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetCurFile) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . map (| | result__) } } }
};
}
