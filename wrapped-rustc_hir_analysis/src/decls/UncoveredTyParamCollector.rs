macro_rules! UncoveredTyParamCollector {
    () => {
        struct UncoveredTyParamCollector < 'cx , 'tcx > { infcx : & 'cx InferCtxt < 'tcx > , uncovered_params : FxIndexSet < DefId > , }
    };
}

UncoveredTyParamCollector!();