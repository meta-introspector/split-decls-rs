macro_rules! deps {
    () => {
        FallbackS!();
        IeeeFloat!();
    };
}

macro_rules! Fallback {
    () => {
        deps!();
        type Fallback < F > = ieee :: IeeeFloat < FallbackS < F > > ;
    };
}

Fallback!()