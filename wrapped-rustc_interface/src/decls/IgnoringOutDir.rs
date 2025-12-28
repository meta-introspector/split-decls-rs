macro_rules! IgnoringOutDir {
    () => {
        # [derive (Diagnostic)] # [diag (interface_ignoring_out_dir)] pub struct IgnoringOutDir ;
    };
}

IgnoringOutDir!()