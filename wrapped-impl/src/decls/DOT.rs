macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! DOT {
    () => {
        deps!();
        static DOT : [(Input , Action) ; 3] = [(Keyword ("await") , SetState (& POSTFIX)) , (ConsumeIdent , SetState (& METHOD)) , (ConsumeLiteral , SetState (& POSTFIX)) ,] ;
    };
}

DOT!();