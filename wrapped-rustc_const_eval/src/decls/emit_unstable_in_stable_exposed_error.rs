macro_rules! deps {
    () => {
        UnstableInStableExposed!();
        ConstCx!();
    };
}

macro_rules! emit_unstable_in_stable_exposed_error {
    () => {
        deps!();
        fn emit_unstable_in_stable_exposed_error (ccx : & ConstCx < '_ , '_ > , span : Span , gate : Symbol , is_function_call : bool ,) -> ErrorGuaranteed { let attr_span = ccx . tcx . def_span (ccx . def_id ()) . shrink_to_lo () ; ccx . dcx () . emit_err (errors :: UnstableInStableExposed { gate : gate . to_string () , span , attr_span , is_function_call , is_function_call2 : is_function_call , }) }
    };
}

emit_unstable_in_stable_exposed_error!();