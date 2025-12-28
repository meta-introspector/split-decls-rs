macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ErrorCreatingImportLibrary {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_error_creating_import_library)] pub (crate) struct ErrorCreatingImportLibrary < 'a > { pub lib_name : & 'a str , pub error : String , }
    };
}

ErrorCreatingImportLibrary!()