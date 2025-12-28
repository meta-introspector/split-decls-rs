macro_rules! deps {
    () => {
        HirDatabase!();
        InferenceResult!();
    };
}

macro_rules! infer_cycle_result {
    () => {
        deps!();
        pub (crate) fn infer_cycle_result (db : & dyn HirDatabase , _salsa_id : salsa :: Id , def : DefWithBodyId ,) -> Arc < InferenceResult < '_ > > { Arc :: new (InferenceResult { has_errors : true , .. InferenceResult :: new (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) }) }
    };
}

infer_cycle_result!();