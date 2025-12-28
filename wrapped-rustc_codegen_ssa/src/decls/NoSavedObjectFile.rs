macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! NoSavedObjectFile {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_no_saved_object_file)] pub (crate) struct NoSavedObjectFile < 'a > { pub cgu_name : & 'a str , }
    };
}

NoSavedObjectFile!();