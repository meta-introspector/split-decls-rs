macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
    };
}

macro_rules! UniversalRegionsBuilder {
    () => {
        deps!();
        struct UniversalRegionsBuilder < 'infcx , 'tcx > { infcx : & 'infcx BorrowckInferCtxt < 'tcx > , mir_def : LocalDefId , }
    };
}

UniversalRegionsBuilder!()