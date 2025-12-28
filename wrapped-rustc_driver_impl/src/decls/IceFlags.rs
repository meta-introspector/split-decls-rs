macro_rules! IceFlags {
    () => {
        # [derive (Diagnostic)] # [diag (driver_impl_ice_flags)] pub (crate) struct IceFlags { pub flags : String , }
    };
}

IceFlags!()