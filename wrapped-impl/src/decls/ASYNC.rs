macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! ASYNC {
    () => {
        deps!();
        static ASYNC : [(Input , Action) ; 3] = [(Keyword ("move") , SetState (& ASYNC)) , (Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeBrace , SetState (& POSTFIX)) ,] ;
    };
}

ASYNC!();