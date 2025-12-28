macro_rules! IceVersion {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_ice_version)] pub (crate) struct IceVersion < 'a > { pub version : & 'a str , pub triple : & 'a str , }
    };
}

IceVersion!()