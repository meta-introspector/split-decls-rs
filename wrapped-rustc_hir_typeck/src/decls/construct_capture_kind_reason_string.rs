macro_rules! construct_capture_kind_reason_string {
    () => {
        fn construct_capture_kind_reason_string < 'tcx > (tcx : TyCtxt < '_ > , place : & Place < 'tcx > , capture_info : & ty :: CaptureInfo ,) -> String { let place_str = construct_place_string (tcx , place) ; let capture_kind_str = match capture_info . capture_kind { ty :: UpvarCapture :: ByValue => "ByValue" . into () , ty :: UpvarCapture :: ByUse => "ByUse" . into () , ty :: UpvarCapture :: ByRef (kind) => format ! ("{kind:?}") , } ; format ! ("{place_str} captured as {capture_kind_str} here") }
    };
}

construct_capture_kind_reason_string!();