macro_rules! borrowed_data_escapes_closure {
    () => {
        pub (crate) fn borrowed_data_escapes_closure < 'tcx > (tcx : TyCtxt < 'tcx > , escape_span : Span , escapes_from : & str ,) -> Diag < 'tcx > { struct_span_code_err ! (tcx . dcx () , escape_span , E0521 , "borrowed data escapes outside of {}" , escapes_from ,) }
    };
}

borrowed_data_escapes_closure!();