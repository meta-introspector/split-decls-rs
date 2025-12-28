macro_rules! deps {
    () => {
        InterpretationResult!();
        CompileTimeMachine!();
        InterpCx!();
        MPlaceTy!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'tcx > InterpretationResult < 'tcx > for ConstAlloc < 'tcx > { fn make_result (mplace : MPlaceTy < 'tcx > , _ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self { ConstAlloc { alloc_id : mplace . ptr () . provenance . unwrap () . alloc_id () , ty : mplace . layout . ty } } }
    };
}

impl_102!()