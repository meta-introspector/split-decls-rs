macro_rules! deps {
    () => {
        VisiblyUninhabited!();
    };
}

macro_rules! CONTINUE_OPAQUELY_INHABITED {
    () => {
        deps!();
        const CONTINUE_OPAQUELY_INHABITED : ControlFlow < VisiblyUninhabited > = Continue (()) ;
    };
}

CONTINUE_OPAQUELY_INHABITED!();