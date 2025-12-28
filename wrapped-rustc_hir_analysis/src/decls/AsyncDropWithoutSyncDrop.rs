macro_rules! AsyncDropWithoutSyncDrop {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_async_drop_without_sync_drop)] # [help] pub (crate) struct AsyncDropWithoutSyncDrop { # [primary_span] pub span : Span , }
    };
}

AsyncDropWithoutSyncDrop!()