// Generated macro for impl_90 (impl)
macro_rules! Depcrate_errorimpl_90 {
() => {
// Module: crate::error
// Provides: {"impl_90"}
// Dependencies: {}
impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { use Error :: * ; match self { LoadLibraryExW { source } | GetModuleHandleExW { source } | GetProcAddress { source } | FreeLibrary { source } => Some (source) , DlOpen { source } | DlSym { source } | DlClose { source } => Some (source) , DlOpenUnknown | DlSymUnknown | DlCloseUnknown | LoadLibraryExWUnknown | GetModuleHandleExWUnknown | GetProcAddressUnknown | FreeLibraryUnknown | IncompatibleSize | InteriorZeroElements => None , } } }
};
}
