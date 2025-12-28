macro_rules! HirIdentifiedAnn {
    () => {
        struct HirIdentifiedAnn < 'tcx > { tcx : TyCtxt < 'tcx > , }
    };
}

HirIdentifiedAnn!()