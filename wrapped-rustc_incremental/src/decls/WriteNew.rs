macro_rules! WriteNew {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_write_new)] pub (crate) struct WriteNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

WriteNew!();