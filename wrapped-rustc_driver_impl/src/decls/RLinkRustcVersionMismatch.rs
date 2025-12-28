macro_rules! RLinkRustcVersionMismatch {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_rlink_rustc_version_mismatch)] pub (crate) struct RLinkRustcVersionMismatch < 'a > { pub rustc_version : String , pub current_version : & 'a str , }
    };
}

RLinkRustcVersionMismatch!();