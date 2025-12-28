macro_rules! eval_to_const_value_raw_provider {
    () => {
        # [instrument (skip (tcx) , level = "debug")] pub fn eval_to_const_value_raw_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> :: rustc_middle :: mir :: interpret :: EvalToConstValueResult < 'tcx > { tcx . eval_to_allocation_raw (key) . map (| val | turn_into_const_value (tcx , val , key)) }
    };
}

eval_to_const_value_raw_provider!();