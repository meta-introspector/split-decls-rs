macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! BREAK_VALUE {
    () => {
        deps!();
        static BREAK_VALUE : [(Input , Action) ; 3] = [(ConsumeNestedBrace , SetState (& IF_THEN)) , (CanBeginExpr , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

BREAK_VALUE!();