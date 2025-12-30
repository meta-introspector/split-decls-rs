// Generated macro for IPersistFile_Impl (trait)
macro_rules! Depcrate_interface_cpp_deriveIPersistFile_Impl {
() => {
// Module: crate::interface_cpp_derive
// Provides: {"IPersistFile_Impl"}
// Dependencies: {}
pub trait IPersistFile_Impl : IPersist_Impl { fn IsDirty (& self) -> windows_core :: HRESULT ; fn Save (& self , pszfilename : & windows_core :: PCWSTR , fremember : windows_core :: BOOL ,) -> windows_core :: Result < () > ; fn SaveCompleted (& self , pszfilename : & windows_core :: PCWSTR) -> windows_core :: Result < () > ; fn GetCurFile (& self) -> windows_core :: Result < windows_core :: PWSTR > ; }
};
}
