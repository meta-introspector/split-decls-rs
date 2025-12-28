macro_rules! CaptureArgLabel {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum CaptureArgLabel { # [label (borrowck_value_capture_here)] Capture { is_within : bool , # [primary_span] args_span : Span , } , # [label (borrowck_move_out_place_here)] MoveOutPlace { place : String , # [primary_span] args_span : Span , } , }
    };
}

CaptureArgLabel!()