macro_rules! deps {
    () => {
        AbiCustomClothedFunction!();
    };
}

macro_rules! check_custom_abi {
    () => {
        deps!();
        pub fn check_custom_abi (tcx : TyCtxt < '_ > , def_id : LocalDefId , fn_sig : FnSig < '_ > , fn_sig_span : Span) { if fn_sig . abi == ExternAbi :: Custom { if ! find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: Naked (_)) { tcx . dcx () . emit_err (crate :: errors :: AbiCustomClothedFunction { span : fn_sig_span , naked_span : tcx . def_span (def_id) . shrink_to_lo () , }) ; } } }
    };
}

check_custom_abi!()