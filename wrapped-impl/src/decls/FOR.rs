macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! FOR {
    () => {
        deps!();
        static FOR : [(Input , Action) ; 2] = [(Punct ("<") , SetState (& CLOSURE)) , (Otherwise , SetState (& PATTERN)) ,] ;
    };
}

FOR!()