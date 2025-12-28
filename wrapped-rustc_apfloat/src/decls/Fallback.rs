macro_rules! deps {
    () => {
        IeeeFloat!();
        FallbackS!();
    };
}

macro_rules! Fallback {
    () => {
        deps!();
        type Fallback < F > = ieee :: IeeeFloat < FallbackS < F > > ;
    };
}

Fallback!();