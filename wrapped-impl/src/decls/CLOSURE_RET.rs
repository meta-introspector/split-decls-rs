macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! CLOSURE_RET {
    () => {
        deps!();
        static CLOSURE_RET : [(Input , Action) ; 2] = [(Punct ("->") , SetState (& [(ExpectType , SetState (& BLOCK))])) , (Otherwise , SetState (& INIT)) ,] ;
    };
}

CLOSURE_RET!();