macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { use Error :: * ; match self { LoadLibraryExW { source } | GetModuleHandleExW { source } | GetProcAddress { source } | FreeLibrary { source } => Some (source) , DlOpen { source } | DlSym { source } | DlClose { source } => Some (source) , DlOpenUnknown | DlSymUnknown | DlCloseUnknown | LoadLibraryExWUnknown | GetModuleHandleExWUnknown | GetProcAddressUnknown | FreeLibraryUnknown | IncompatibleSize | InteriorZeroElements => None , } } }
    };
}

impl_78!();