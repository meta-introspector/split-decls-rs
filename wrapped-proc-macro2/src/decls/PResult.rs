macro_rules! deps {
    () => {
        Reject!();
        Cursor!();
    };
}

macro_rules! PResult {
    () => {
        deps!();
        type PResult < 'a , O > = Result < (Cursor < 'a > , O) , Reject > ;
    };
}

PResult!();