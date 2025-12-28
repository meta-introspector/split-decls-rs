macro_rules! deps {
    () => {
        ChallengeParamRef!();
    };
}

macro_rules! ParamsPrinter {
    () => {
        deps!();
        struct ParamsPrinter < 'i > (& 'i [ChallengeParamRef < 'i >]) ;
    };
}

ParamsPrinter!();