macro_rules! deps {
    () => {
        InferCtxt!();
        SnapshotVarData!();
    };
}

macro_rules! InferenceFudger {
    () => {
        deps!();
        struct InferenceFudger < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , snapshot_vars : SnapshotVarData , }
    };
}

InferenceFudger!()