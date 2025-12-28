macro_rules! LoadProcMacroDylibError {
    () => {
        # [derive (Debug)] pub enum LoadProcMacroDylibError { Io (io :: Error) , LibLoading (libloading :: Error) , AbiMismatch (String) , }
    };
}

LoadProcMacroDylibError!()