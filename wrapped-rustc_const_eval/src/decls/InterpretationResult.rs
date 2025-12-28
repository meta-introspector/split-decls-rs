macro_rules! deps {
    () => {
        MPlaceTy!();
        InterpCx!();
        CompileTimeMachine!();
    };
}

macro_rules! InterpretationResult {
    () => {
        deps!();
        pub trait InterpretationResult < 'tcx > { # [doc = " This function takes the place where the result of the evaluation is stored"] # [doc = " and prepares it for returning it in the appropriate format needed by the specific"] # [doc = " evaluation query."] fn make_result (mplace : MPlaceTy < 'tcx > , ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self ; }
    };
}

InterpretationResult!();