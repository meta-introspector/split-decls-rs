macro_rules! deps {
    () => {
        InterpCx!();
        MPlaceTy!();
        CompileTimeMachine!();
        InterpretationResult!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < 'tcx > InterpretationResult < 'tcx > for mir :: interpret :: ConstAllocation < 'tcx > { fn make_result (mplace : MPlaceTy < 'tcx > , ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self { let alloc_id = mplace . ptr () . provenance . unwrap () . alloc_id () ; let alloc = ecx . memory . alloc_map . swap_remove (& alloc_id) . unwrap () . 1 ; ecx . tcx . mk_const_alloc (alloc) } }
    };
}

impl_335!();