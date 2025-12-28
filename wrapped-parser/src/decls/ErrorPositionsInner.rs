macro_rules! deps {
    () => {
        Pos!();
    };
}

macro_rules! ErrorPositionsInner {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] enum ErrorPositionsInner { Two (Pos , Pos) , One (Pos) , None , }
    };
}

ErrorPositionsInner!();