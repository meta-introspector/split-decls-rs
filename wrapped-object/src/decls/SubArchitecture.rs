macro_rules! SubArchitecture {
    () => {
        # [doc = " A CPU sub-architecture."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SubArchitecture { Arm64E , Arm64EC , }
    };
}

SubArchitecture!()