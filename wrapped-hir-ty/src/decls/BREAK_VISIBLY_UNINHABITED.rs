macro_rules! deps {
    () => {
        VisiblyUninhabited!();
    };
}

macro_rules! BREAK_VISIBLY_UNINHABITED {
    () => {
        deps!();
        const BREAK_VISIBLY_UNINHABITED : ControlFlow < VisiblyUninhabited > = Break (VisiblyUninhabited) ;
    };
}

BREAK_VISIBLY_UNINHABITED!();