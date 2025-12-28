macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! METHOD {
    () => {
        deps!();
        static METHOD : [(Input , Action) ; 1] = [(ExpectTurbofish , SetState (& POSTFIX))] ;
    };
}

METHOD!();