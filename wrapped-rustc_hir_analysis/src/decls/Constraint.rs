macro_rules! deps {
    () => {
        VarianceTerm!();
        InferredIndex!();
    };
}

macro_rules! Constraint {
    () => {
        deps!();
        # [doc = " Declares that the variable `decl_id` appears in a location with"] # [doc = " variance `variance`."] # [derive (Copy , Clone)] pub (crate) struct Constraint < 'a > { pub inferred : InferredIndex , pub variance : & 'a VarianceTerm < 'a > , }
    };
}

Constraint!();