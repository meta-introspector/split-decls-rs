macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! uninhabited {
    () => {
        deps!();
        # [test] fn uninhabited () { enum Void { } let _sv = SmallVec :: < Void , 8 > :: new () ; }
    };
}

uninhabited!()