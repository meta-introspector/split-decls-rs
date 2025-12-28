macro_rules! deps {
    () => {
        IllegalMoveOriginKind!();
        MoveError!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'tcx > MoveError < 'tcx > { pub (crate) fn new (place : Place < 'tcx > , location : Location , kind : IllegalMoveOriginKind < 'tcx > ,) -> Self { MoveError { place , location , kind } } }
    };
}

impl_132!();