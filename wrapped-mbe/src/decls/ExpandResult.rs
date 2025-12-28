macro_rules! deps {
    () => {
        ValueResult!();
        ExpandError!();
    };
}

macro_rules! ExpandResult {
    () => {
        deps!();
        pub type ExpandResult < T > = ValueResult < T , ExpandError > ;
    };
}

ExpandResult!();