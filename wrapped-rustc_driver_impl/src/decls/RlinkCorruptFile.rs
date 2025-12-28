macro_rules! RlinkCorruptFile {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_corrupt_file)] pub (crate) struct RlinkCorruptFile < 'a > { pub file : & 'a std :: path :: Path , }
    };
}

RlinkCorruptFile!()