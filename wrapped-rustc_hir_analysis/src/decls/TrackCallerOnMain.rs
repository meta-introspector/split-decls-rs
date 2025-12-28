macro_rules! TrackCallerOnMain {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_track_caller_on_main)] pub (crate) struct TrackCallerOnMain { # [primary_span] # [suggestion (applicability = "maybe-incorrect" , code = "")] pub span : Span , # [label (hir_analysis_track_caller_on_main)] pub annotated : Span , }
    };
}

TrackCallerOnMain!()