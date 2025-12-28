macro_rules! deps {
    () => {
        MethodError!();
        Pick!();
    };
}

macro_rules! PickResult {
    () => {
        deps!();
        pub (crate) type PickResult < 'tcx > = Result < Pick < 'tcx > , MethodError < 'tcx > > ;
    };
}

PickResult!()