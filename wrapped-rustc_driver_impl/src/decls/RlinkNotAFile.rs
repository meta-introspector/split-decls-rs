macro_rules! RlinkNotAFile {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_no_a_file)] pub (crate) struct RlinkNotAFile ;
    };
}

RlinkNotAFile!();