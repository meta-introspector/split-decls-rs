macro_rules! SessionGcFailed {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_session_gc_failed)] pub (crate) struct SessionGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

SessionGcFailed!()