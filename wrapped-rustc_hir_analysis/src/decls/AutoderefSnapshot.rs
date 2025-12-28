macro_rules! deps {
    () => {
        AutoderefKind!();
    };
}

macro_rules! AutoderefSnapshot {
    () => {
        deps!();
        struct AutoderefSnapshot < 'tcx > { at_start : bool , reached_recursion_limit : bool , steps : Vec < (Ty < 'tcx > , AutoderefKind) > , cur_ty : Ty < 'tcx > , obligations : PredicateObligations < 'tcx > , }
    };
}

AutoderefSnapshot!();