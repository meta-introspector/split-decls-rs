// Generated macro for impl_91 (impl)
macro_rules! Depcrate_errorimpl_91 {
() => {
// Module: crate::error
// Provides: {"impl_91"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use Error :: * ; match * self { DlOpen { .. } => write ! (f , "dlopen failed") , DlOpenUnknown => write ! (f , "dlopen failed, but system did not report the error") , DlSym { .. } => write ! (f , "dlsym failed") , DlSymUnknown => write ! (f , "dlsym failed, but system did not report the error") , DlClose { .. } => write ! (f , "dlclose failed") , DlCloseUnknown => write ! (f , "dlclose failed, but system did not report the error") , LoadLibraryExW { .. } => write ! (f , "LoadLibraryExW failed") , LoadLibraryExWUnknown => write ! (f , "LoadLibraryExW failed, but system did not report the error") , GetModuleHandleExW { .. } => write ! (f , "GetModuleHandleExW failed") , GetModuleHandleExWUnknown => write ! (f , "GetModuleHandleExWUnknown failed, but system did not report the error") , GetProcAddress { .. } => write ! (f , "GetProcAddress failed") , GetProcAddressUnknown => write ! (f , "GetProcAddress failed, but system did not report the error") , FreeLibrary { .. } => write ! (f , "FreeLibrary failed") , FreeLibraryUnknown => { write ! (f , "FreeLibrary failed, but system did not report the error") } InteriorZeroElements => write ! (f , "interior zero element in parameter") , IncompatibleSize => write ! (f , "requested type cannot possibly work") , } } }
};
}
