macro_rules! deps {
    () => {
        PerLocalVarDebugInfo!();
    };
}

macro_rules! PerLocalVarDebugInfoIndexVec {
    () => {
        deps!();
        type PerLocalVarDebugInfoIndexVec < 'tcx , V > = IndexVec < mir :: Local , Vec < PerLocalVarDebugInfo < 'tcx , V > > > ;
    };
}

PerLocalVarDebugInfoIndexVec!()