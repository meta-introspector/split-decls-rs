macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! RETURN {
    () => {
        deps!();
        static RETURN : [(Input , Action) ; 2] = [(CanBeginExpr , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

RETURN!();