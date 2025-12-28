macro_rules! InherentCollect {
    () => {
        struct InherentCollect < 'tcx > { tcx : TyCtxt < 'tcx > , impls_map : CrateInherentImpls , }
    };
}

InherentCollect!()