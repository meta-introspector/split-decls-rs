macro_rules! deps {
    () => {
        CompileTimeMachine!();
        RawBytesNote!();
        InterpCx!();
    };
}

macro_rules! report_validation_error {
    () => {
        deps!();
        # [inline (never)] fn report_validation_error < 'tcx > (ecx : & InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , cid : GlobalId < 'tcx > , error : InterpErrorInfo < 'tcx > , alloc_id : AllocId ,) -> ErrorHandled { if ! matches ! (error . kind () , InterpErrorKind :: UndefinedBehavior (_)) { return report_eval_error (ecx , cid , error) ; } let (error , backtrace) = error . into_parts () ; backtrace . print_backtrace () ; let bytes = ecx . print_alloc_bytes_for_diagnostics (alloc_id) ; let info = ecx . get_alloc_info (alloc_id) ; let raw_bytes = errors :: RawBytesNote { size : info . size . bytes () , align : info . align . bytes () , bytes } ; crate :: const_eval :: report (ecx , error , DUMMY_SP , | | crate :: const_eval :: get_span_and_frames (ecx . tcx , ecx . stack ()) , move | diag , span , frames | { diag . code (E0080) ; diag . span_label (span , crate :: fluent_generated :: const_eval_validation_failure) ; diag . note (crate :: fluent_generated :: const_eval_validation_failure_note) ; for frame in frames { diag . subdiagnostic (frame) ; } diag . subdiagnostic (raw_bytes) ; } ,) }
    };
}

report_validation_error!()