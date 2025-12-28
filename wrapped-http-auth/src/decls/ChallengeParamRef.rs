macro_rules! deps {
    () => {
        ParamValue!();
    };
}

macro_rules! ChallengeParamRef {
    () => {
        deps!();
        type ChallengeParamRef < 'i > = (& 'i str , ParamValue < 'i >) ;
    };
}

ChallengeParamRef!()