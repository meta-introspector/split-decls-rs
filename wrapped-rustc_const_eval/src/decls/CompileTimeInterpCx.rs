macro_rules! deps {
    () => {
        InterpCx!();
        CompileTimeMachine!();
    };
}

macro_rules! CompileTimeInterpCx {
    () => {
        deps!();
        pub type CompileTimeInterpCx < 'tcx > = InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ;
    };
}

CompileTimeInterpCx!();