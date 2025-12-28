macro_rules! deps {
    () => {
        ExpandError!();
        ValueResult!();
    };
}

macro_rules! ExpandResult {
    () => {
        deps!();
        pub type ExpandResult < T > = ValueResult < T , ExpandError > ;
    };
}

ExpandResult!()