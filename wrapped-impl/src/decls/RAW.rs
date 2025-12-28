macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! RAW {
    () => {
        deps!();
        static RAW : [(Input , Action) ; 3] = [(Keyword ("const") , SetState (& INIT)) , (Keyword ("mut") , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
    };
}

RAW!();