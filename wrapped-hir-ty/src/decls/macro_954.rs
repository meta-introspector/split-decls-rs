macro_rules! deps {
    () => {
        CandidateId!();
    };
}

macro_rules! macro_954 {
    () => {
        deps!();
        impl_from ! (FunctionId , ConstId for CandidateId) ;
    };
}

macro_954!();