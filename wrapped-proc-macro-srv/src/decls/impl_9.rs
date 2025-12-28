macro_rules! deps {
    () => {
        LoadProcMacroDylibError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Display for LoadProcMacroDylibError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Io (e) => e . fmt (f) , Self :: AbiMismatch (v) => { use crate :: RUSTC_VERSION_STRING ; write ! (f , "mismatched ABI expected: `{RUSTC_VERSION_STRING}`, got `{v}`") } Self :: LibLoading (e) => e . fmt (f) , } } }
    };
}

impl_9!();