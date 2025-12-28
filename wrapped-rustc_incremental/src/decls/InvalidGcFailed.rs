macro_rules! InvalidGcFailed {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_invalid_gc_failed)] pub (crate) struct InvalidGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

InvalidGcFailed!();