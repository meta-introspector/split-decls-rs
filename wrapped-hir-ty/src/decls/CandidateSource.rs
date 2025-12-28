macro_rules! CandidateSource {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum CandidateSource { Impl (ImplId) , Trait (TraitId) , }
    };
}

CandidateSource!();