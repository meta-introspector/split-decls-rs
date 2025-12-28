macro_rules! deps {
    () => {
        IllegalMoveOriginKind!();
    };
}

macro_rules! MoveError {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct MoveError < 'tcx > { place : Place < 'tcx > , location : Location , kind : IllegalMoveOriginKind < 'tcx > , }
    };
}

MoveError!();