macro_rules! deps {
    () => {
        CoerceResult!();
    };
}

macro_rules! success {
    () => {
        deps!();
        # [doc = " This always returns `Ok(...)`."] fn success < 'tcx > (adj : Vec < Adjustment < 'tcx > > , target : Ty < 'tcx > , obligations : PredicateObligations < 'tcx > ,) -> CoerceResult < 'tcx > { Ok (InferOk { value : (adj , target) , obligations }) }
    };
}

success!()