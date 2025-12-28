macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! RANGE {
    () => {
        deps!();
        static RANGE : [(Input , Action) ; 6] = [(Punct ("..=") , SetState (& INIT)) , (Punct ("..") , SetState (& RANGE)) , (Punct (".") , SetState (& DOT)) , (ConsumeNestedBrace , SetState (& IF_THEN)) , (Empty , Finish) , (Otherwise , SetState (& INIT)) ,] ;
    };
}

RANGE!();