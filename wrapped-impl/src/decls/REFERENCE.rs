macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! REFERENCE {
    () => {
        deps!();
        static REFERENCE : [(Input , Action) ; 3] = [(Keyword ("mut") , SetState (& INIT)) , (Keyword ("raw") , SetState (& RAW)) , (Otherwise , SetState (& INIT)) ,] ;
    };
}

REFERENCE!();