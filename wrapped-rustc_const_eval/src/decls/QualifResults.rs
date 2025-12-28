macro_rules! deps {
    () => {
        FlowSensitiveAnalysis!();
    };
}

macro_rules! QualifResults {
    () => {
        deps!();
        type QualifResults < 'mir , 'tcx , Q > = rustc_mir_dataflow :: ResultsCursor < 'mir , 'tcx , FlowSensitiveAnalysis < 'mir , 'tcx , Q > > ;
    };
}

QualifResults!();