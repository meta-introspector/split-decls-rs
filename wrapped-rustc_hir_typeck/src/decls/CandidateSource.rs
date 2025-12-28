macro_rules! CandidateSource {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum CandidateSource { Impl (DefId) , Trait (DefId) , }
    };
}

CandidateSource!();