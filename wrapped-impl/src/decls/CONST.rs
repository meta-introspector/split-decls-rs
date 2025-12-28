macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! CONST {
    () => {
        deps!();
        static CONST : [(Input , Action) ; 2] = [(Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeBrace , SetState (& POSTFIX)) ,] ;
    };
}

CONST!()