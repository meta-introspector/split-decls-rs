macro_rules! CrateNameInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (interface_crate_name_invalid)] pub (crate) struct CrateNameInvalid < 'a > { pub (crate) crate_name : & 'a str , }
    };
}

CrateNameInvalid!();