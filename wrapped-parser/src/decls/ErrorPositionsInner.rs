macro_rules! ErrorPositionsInner {
    () => {
        # [derive (Debug , Clone , Copy)] enum ErrorPositionsInner { Two (Pos , Pos) , One (Pos) , None , }
    };
}

ErrorPositionsInner!()