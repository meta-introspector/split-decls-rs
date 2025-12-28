macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! CLOSURE_ARGS {
    () => {
        deps!();
        static CLOSURE_ARGS : [(Input , Action) ; 2] = [(Punct ("|") , SetState (& CLOSURE_RET)) , (ConsumeAny , SetState (& CLOSURE_ARGS)) ,] ;
    };
}

CLOSURE_ARGS!();