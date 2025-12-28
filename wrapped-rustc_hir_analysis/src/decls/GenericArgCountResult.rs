macro_rules! deps {
    () => {
        ExplicitLateBound!();
        GenericArgCountMismatch!();
    };
}

macro_rules! GenericArgCountResult {
    () => {
        deps!();
        # [doc = " Decorates the result of a generic argument count mismatch"] # [doc = " check with whether explicit late bounds were provided."] # [derive (Clone , Debug)] pub struct GenericArgCountResult { pub explicit_late_bound : ExplicitLateBound , pub correct : Result < () , GenericArgCountMismatch > , }
    };
}

GenericArgCountResult!()