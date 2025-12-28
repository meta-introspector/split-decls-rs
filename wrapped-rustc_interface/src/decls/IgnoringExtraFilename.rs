macro_rules! IgnoringExtraFilename {
    () => {
        # [derive (Diagnostic)] # [diag (interface_ignoring_extra_filename)] pub struct IgnoringExtraFilename ;
    };
}

IgnoringExtraFilename!();