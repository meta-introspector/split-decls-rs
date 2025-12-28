macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CguNotRecorded {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_cgu_not_recorded)] pub (crate) struct CguNotRecorded < 'a > { pub cgu_user_name : & 'a str , pub cgu_name : & 'a str , }
    };
}

CguNotRecorded!();