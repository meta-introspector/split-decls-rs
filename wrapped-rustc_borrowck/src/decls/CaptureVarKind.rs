macro_rules! CaptureVarKind {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum CaptureVarKind { # [label (borrowck_capture_immute)] Immut { # [primary_span] kind_span : Span , } , # [label (borrowck_capture_mut)] Mut { # [primary_span] kind_span : Span , } , # [label (borrowck_capture_move)] Move { # [primary_span] kind_span : Span , } , }
    };
}

CaptureVarKind!();