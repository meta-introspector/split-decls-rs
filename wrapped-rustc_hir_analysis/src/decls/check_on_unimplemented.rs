macro_rules! check_on_unimplemented {
    () => {
        pub (super) fn check_on_unimplemented (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let _ = OnUnimplementedDirective :: of_item (tcx , def_id . to_def_id ()) ; }
    };
}

check_on_unimplemented!();