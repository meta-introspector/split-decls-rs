macro_rules! deps {
    () => {
        MPlaceTy!();
        PlaceTy!();
        CompileTimeInterpCx!();
    };
}

macro_rules! dump_place {
    () => {
        deps!();
        fn dump_place < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx >) { trace ! ("{:?}" , ecx . dump_place (& PlaceTy :: from (place . clone ()))) ; }
    };
}

dump_place!();