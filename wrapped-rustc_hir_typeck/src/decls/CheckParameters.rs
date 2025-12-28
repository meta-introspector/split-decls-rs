macro_rules! CheckParameters {
    () => {
        struct CheckParameters < 'tcx > { tcx : TyCtxt < 'tcx > , params : HirIdSet , }
    };
}

CheckParameters!()