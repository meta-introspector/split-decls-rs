macro_rules! CandidateId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum CandidateId { FunctionId (FunctionId) , ConstId (ConstId) , }
    };
}

CandidateId!()