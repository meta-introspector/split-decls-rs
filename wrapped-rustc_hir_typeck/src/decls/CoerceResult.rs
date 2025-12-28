macro_rules! CoerceResult {
    () => {
        type CoerceResult < 'tcx > = InferResult < 'tcx , (Vec < Adjustment < 'tcx > > , Ty < 'tcx >) > ;
    };
}

CoerceResult!();