macro_rules! deps {
    () => {
        HirTyLowerer!();
    };
}

macro_rules! GenericParamAndBoundVarCollector {
    () => {
        deps!();
        struct GenericParamAndBoundVarCollector < 'a , 'tcx > { cx : & 'a dyn HirTyLowerer < 'tcx > , params : FxIndexSet < u32 > , vars : FxIndexSet < DefId > , depth : ty :: DebruijnIndex , }
    };
}

GenericParamAndBoundVarCollector!();