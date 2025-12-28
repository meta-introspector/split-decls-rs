macro_rules! is_from_coerce_pointee_derive {
    () => {
        fn is_from_coerce_pointee_derive (tcx : TyCtxt < '_ > , span : Span) -> bool { span . ctxt () . outer_expn_data () . macro_def_id . is_some_and (| def_id | tcx . is_diagnostic_item (sym :: CoercePointee , def_id)) }
    };
}

is_from_coerce_pointee_derive!();