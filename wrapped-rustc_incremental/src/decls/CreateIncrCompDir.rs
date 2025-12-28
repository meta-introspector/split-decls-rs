macro_rules! CreateIncrCompDir {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_create_incr_comp_dir)] pub (crate) struct CreateIncrCompDir < 'a > { pub tag : & 'a str , pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

CreateIncrCompDir!();