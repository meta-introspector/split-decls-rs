macro_rules! deps {
    () => {
        TooLargeStatic!();
    };
}

macro_rules! check_static_inhabited {
    () => {
        deps!();
        # [doc = " Check that a `static` is inhabited."] fn check_static_inhabited (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let ty = tcx . type_of (def_id) . instantiate_identity () ; let span = tcx . def_span (def_id) ; let layout = match tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (ty)) { Ok (l) => l , Err (LayoutError :: SizeOverflow (_)) if matches ! (tcx . def_kind (def_id) , DefKind :: Static { .. } if tcx . def_kind (tcx . local_parent (def_id)) == DefKind :: ForeignMod) => { tcx . dcx () . emit_err (errors :: TooLargeStatic { span }) ; return ; } Err (e) => { tcx . dcx () . span_delayed_bug (span , format ! ("{e:?}")) ; return ; } } ; if layout . is_uninhabited () { tcx . node_span_lint (UNINHABITED_STATIC , tcx . local_def_id_to_hir_id (def_id) , span , | lint | { lint . primary_message ("static of uninhabited type") ; lint . note ("uninhabited statics cannot be initialized, and any access would be an immediate error") ; } ,) ; } }
    };
}

check_static_inhabited!();