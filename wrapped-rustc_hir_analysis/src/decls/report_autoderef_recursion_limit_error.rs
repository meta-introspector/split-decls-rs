macro_rules! deps {
    () => {
        AutoDerefReachedRecursionLimit!();
    };
}

macro_rules! report_autoderef_recursion_limit_error {
    () => {
        deps!();
        pub fn report_autoderef_recursion_limit_error < 'tcx > (tcx : TyCtxt < 'tcx > , span : Span , ty : Ty < 'tcx > ,) -> ErrorGuaranteed { let suggested_limit = match tcx . recursion_limit () { Limit (0) => Limit (2) , limit => limit * 2 , } ; tcx . dcx () . emit_err (AutoDerefReachedRecursionLimit { span , ty , suggested_limit , crate_name : tcx . crate_name (LOCAL_CRATE) , }) }
    };
}

report_autoderef_recursion_limit_error!()