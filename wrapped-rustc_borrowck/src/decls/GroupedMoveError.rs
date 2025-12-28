macro_rules! deps {
    () => {
        IllegalMoveOriginKind!();
        UseSpans!();
    };
}

macro_rules! GroupedMoveError {
    () => {
        deps!();
        # [derive (Debug)] enum GroupedMoveError < 'tcx > { MovesFromPlace { original_path : Place < 'tcx > , span : Span , move_from : Place < 'tcx > , kind : IllegalMoveOriginKind < 'tcx > , binds_to : Vec < Local > , } , MovesFromValue { original_path : Place < 'tcx > , span : Span , move_from : MovePathIndex , kind : IllegalMoveOriginKind < 'tcx > , binds_to : Vec < Local > , } , OtherIllegalMove { original_path : Place < 'tcx > , use_spans : UseSpans < 'tcx > , kind : IllegalMoveOriginKind < 'tcx > , } , }
    };
}

GroupedMoveError!();