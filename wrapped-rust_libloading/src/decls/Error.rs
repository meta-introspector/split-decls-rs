macro_rules! deps {
    () => {
        WindowsError!();
        DlError!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Errors."] # [derive (Debug)] # [non_exhaustive] pub enum Error { # [doc = " The `dlopen` call failed."] DlOpen { # [doc = " The source error."] source : DlError , } , # [doc = " The `dlopen` call failed and system did not report an error."] DlOpenUnknown , # [doc = " The `dlsym` call failed."] DlSym { # [doc = " The source error."] source : DlError , } , # [doc = " The `dlsym` call failed and system did not report an error."] DlSymUnknown , # [doc = " The `dlclose` call failed."] DlClose { # [doc = " The source error."] source : DlError , } , # [doc = " The `dlclose` call failed and system did not report an error."] DlCloseUnknown , # [doc = " The `LoadLibraryW` call failed."] LoadLibraryExW { # [doc = " The source error."] source : WindowsError , } , # [doc = " The `LoadLibraryW` call failed and system did not report an error."] LoadLibraryExWUnknown , # [doc = " The `GetModuleHandleExW` call failed."] GetModuleHandleExW { # [doc = " The source error."] source : WindowsError , } , # [doc = " The `GetModuleHandleExW` call failed and system did not report an error."] GetModuleHandleExWUnknown , # [doc = " The `GetProcAddress` call failed."] GetProcAddress { # [doc = " The source error."] source : WindowsError , } , # [doc = " The `GetProcAddressUnknown` call failed and system did not report an error."] GetProcAddressUnknown , # [doc = " The `FreeLibrary` call failed."] FreeLibrary { # [doc = " The source error."] source : WindowsError , } , # [doc = " The `FreeLibrary` call failed and system did not report an error."] FreeLibraryUnknown , # [doc = " The requested type cannot possibly work."] IncompatibleSize , # [doc = " Input symbol of filename contains interior 0/null elements."] InteriorZeroElements , }
    };
}

Error!()