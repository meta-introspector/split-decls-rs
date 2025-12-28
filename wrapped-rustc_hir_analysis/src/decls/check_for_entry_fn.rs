macro_rules! check_for_entry_fn {
    () => {
        pub (crate) fn check_for_entry_fn (tcx : TyCtxt < '_ >) { match tcx . entry_fn (()) { Some ((def_id , EntryFnType :: Main { .. })) => check_main_fn_ty (tcx , def_id) , _ => { } } }
    };
}

check_for_entry_fn!();