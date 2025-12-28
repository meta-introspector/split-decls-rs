macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! CLOSURE {
    () => {
        deps!();
        static CLOSURE : [(Input , Action) ; 6] = [(Keyword ("async") , SetState (& CLOSURE)) , (Keyword ("move") , SetState (& CLOSURE)) , (Punct (",") , SetState (& CLOSURE)) , (Punct (">") , SetState (& CLOSURE)) , (Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeLifetime , SetState (& CLOSURE)) ,] ;
    };
}

CLOSURE!()