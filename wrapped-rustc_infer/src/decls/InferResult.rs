macro_rules! deps {
    () => {
        InferOk!();
    };
}

macro_rules! InferResult {
    () => {
        deps!();
        pub type InferResult < 'tcx , T > = Result < InferOk < 'tcx , T > , TypeError < 'tcx > > ;
    };
}

InferResult!()