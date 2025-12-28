macro_rules! OutDirError {
    () => {
        # [derive (Diagnostic)] # [diag (interface_out_dir_error)] pub struct OutDirError ;
    };
}

OutDirError!()