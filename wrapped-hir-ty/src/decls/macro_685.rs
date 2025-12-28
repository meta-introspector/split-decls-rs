macro_rules! deps {
    () => {
        CandidateId!();
    };
}

macro_rules! macro_685 {
    () => {
        deps!();
        impl_from ! (FunctionId , ConstId for CandidateId) ;
    };
}

macro_685!()