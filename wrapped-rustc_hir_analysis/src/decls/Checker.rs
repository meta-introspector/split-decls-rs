macro_rules! Checker {
    () => {
        struct Checker < 'tcx > { tcx : TyCtxt < 'tcx > , trait_def_id : DefId , impl_def_id : LocalDefId , impl_header : ty :: ImplTraitHeader < 'tcx > , }
    };
}

Checker!()