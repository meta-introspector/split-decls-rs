macro_rules! adt_async_destructor {
    () => {
        fn adt_async_destructor (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: AsyncDestructor > { tcx . calculate_async_dtor (def_id , always_applicable :: check_drop_impl) }
    };
}

adt_async_destructor!();