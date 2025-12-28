macro_rules! deps {
    () => {
        Pick!();
        MethodError!();
    };
}

macro_rules! PickResult {
    () => {
        deps!();
        pub (crate) type PickResult < 'tcx > = Result < Pick < 'tcx > , MethodError < 'tcx > > ;
    };
}

PickResult!();