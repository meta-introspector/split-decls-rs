macro_rules! FinalizedGcFailed {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_finalized_gc_failed)] pub (crate) struct FinalizedGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

FinalizedGcFailed!()