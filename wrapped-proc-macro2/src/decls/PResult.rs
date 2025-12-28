macro_rules! deps {
    () => {
        Cursor!();
        Reject!();
    };
}

macro_rules! PResult {
    () => {
        deps!();
        type PResult < 'a , O > = Result < (Cursor < 'a > , O) , Reject > ;
    };
}

PResult!()