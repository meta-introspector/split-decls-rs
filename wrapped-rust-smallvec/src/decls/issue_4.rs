macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! issue_4 {
    () => {
        deps!();
        # [test] fn issue_4 () { SmallVec :: < Box < u32 > , 2 > :: new () ; }
    };
}

issue_4!();