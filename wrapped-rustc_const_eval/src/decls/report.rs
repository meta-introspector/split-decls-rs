macro_rules! deps {
    () => {
        InterpCx!();
        FrameNote!();
        CompileTimeMachine!();
        RawBytesNote!();
    };
}

macro_rules! report {
    () => {
        deps!();
        # [doc = " Create a diagnostic for a const eval error."] # [doc = ""] # [doc = " This will use the `mk` function for adding more information to the error."] # [doc = " You can use it to add a stacktrace of current execution according to"] # [doc = " `get_span_and_frames` or just give context on where the const eval error happened."] pub (super) fn report < 'tcx , C , F > (ecx : & InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , error : InterpErrorKind < 'tcx > , span : Span , get_span_and_frames : C , mk : F ,) -> ErrorHandled where C : FnOnce () -> (Span , Vec < FrameNote >) , F : FnOnce (& mut Diag < '_ > , Span , Vec < FrameNote >) , { let tcx = ecx . tcx . tcx ; match error { err_inval ! (AlreadyReported (info)) => ErrorHandled :: Reported (info , span) , err_inval ! (Layout (LayoutError :: TooGeneric (_))) | err_inval ! (TooGeneric) => { ErrorHandled :: TooGeneric (span) } err_inval ! (Layout (LayoutError :: ReferencesError (guar))) => { ErrorHandled :: Reported (ReportedErrorInfo :: allowed_in_infallible (guar) , span) } _ => { let (our_span , frames) = get_span_and_frames () ; let span = span . substitute_dummy (our_span) ; let mut err = tcx . dcx () . struct_span_err (our_span , error . diagnostic_message ()) ; let allowed_in_infallible = matches ! (error , InterpErrorKind :: ResourceExhaustion (_) | InterpErrorKind :: InvalidProgram (_)) ; if let InterpErrorKind :: UndefinedBehavior (UndefinedBehaviorInfo :: InvalidUninitBytes (Some ((alloc_id , _access)) ,)) = error { let bytes = ecx . print_alloc_bytes_for_diagnostics (alloc_id) ; let info = ecx . get_alloc_info (alloc_id) ; let raw_bytes = errors :: RawBytesNote { size : info . size . bytes () , align : info . align . bytes () , bytes , } ; err . subdiagnostic (raw_bytes) ; } error . add_args (& mut err) ; mk (& mut err , span , frames) ; let g = err . emit () ; let reported = if allowed_in_infallible { ReportedErrorInfo :: allowed_in_infallible (g) } else { ReportedErrorInfo :: const_eval_error (g) } ; ErrorHandled :: Reported (reported , span) } } }
    };
}

report!();