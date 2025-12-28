macro_rules! RLinkEncodingVersionMismatch {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_encoding_version_mismatch)] pub (crate) struct RLinkEncodingVersionMismatch { pub version_array : String , pub rlink_version : u32 , }
    };
}

RLinkEncodingVersionMismatch!()