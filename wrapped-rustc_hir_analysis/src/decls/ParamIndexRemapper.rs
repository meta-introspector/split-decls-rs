macro_rules! deps {
    () => {
        RemapTable!();
    };
}

macro_rules! ParamIndexRemapper {
    () => {
        deps!();
        struct ParamIndexRemapper < 'tcx > { tcx : TyCtxt < 'tcx > , remap_table : RemapTable , }
    };
}

ParamIndexRemapper!();