macro_rules! deps {
    () => {
        Input!();
        Action!();
    };
}

macro_rules! BLOCK {
    () => {
        deps!();
        static BLOCK : [(Input , Action) ; 1] = [(ConsumeBrace , SetState (& POSTFIX))] ;
    };
}

BLOCK!()