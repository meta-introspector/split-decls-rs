macro_rules! RLinkWrongFileType {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_wrong_file_type)] pub (crate) struct RLinkWrongFileType ;
    };
}

RLinkWrongFileType!()