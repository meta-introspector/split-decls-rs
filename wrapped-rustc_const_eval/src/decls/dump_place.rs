macro_rules! deps {
    () => {
        MPlaceTy!();
        CompileTimeInterpCx!();
        PlaceTy!();
    };
}

macro_rules! dump_place {
    () => {
        deps!();
        fn dump_place < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx >) { trace ! ("{:?}" , ecx . dump_place (& PlaceTy :: from (place . clone ()))) ; }
    };
}

dump_place!()