macro_rules! deps {
    () => {
        Send!();
    };
}

macro_rules! SendMsg {
    () => {
        deps!();
        type SendMsg < Fut > = Result < < Fut as Future > :: Output , Box < dyn Any + Send + 'static > > ;
    };
}

SendMsg!();