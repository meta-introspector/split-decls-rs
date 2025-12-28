macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! AddNativeLibrary {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_add_native_library)] pub (crate) struct AddNativeLibrary { pub library_path : PathBuf , pub error : Error , }
    };
}

AddNativeLibrary!();