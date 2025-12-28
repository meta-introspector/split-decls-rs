macro_rules! TempsDirError {
    () => {
        # [derive (Diagnostic)] # [diag (interface_temps_dir_error)] pub struct TempsDirError ;
    };
}

TempsDirError!()