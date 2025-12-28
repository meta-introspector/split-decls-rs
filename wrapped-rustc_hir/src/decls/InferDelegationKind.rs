macro_rules! InferDelegationKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , HashStable_Generic)] pub enum InferDelegationKind { Input (usize) , Output , }
    };
}

InferDelegationKind!()