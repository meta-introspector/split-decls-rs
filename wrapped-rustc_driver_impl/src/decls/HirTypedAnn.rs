macro_rules! HirTypedAnn {
    () => {
        struct HirTypedAnn < 'tcx > { tcx : TyCtxt < 'tcx > , maybe_typeck_results : Cell < Option < & 'tcx ty :: TypeckResults < 'tcx > > > , }
    };
}

HirTypedAnn!();