macro_rules! RemapLateParam {
    () => {
        struct RemapLateParam < 'tcx > { tcx : TyCtxt < 'tcx > , mapping : FxIndexMap < ty :: LateParamRegionKind , ty :: LateParamRegionKind > , }
    };
}

RemapLateParam!();