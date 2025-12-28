macro_rules! CreateNew {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_create_new)] pub (crate) struct CreateNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

CreateNew!()