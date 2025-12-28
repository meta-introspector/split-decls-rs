macro_rules! deps {
    () => {
        ExpandError!();
    };
}

macro_rules! ExpandResult {
    () => {
        deps!();
        pub type ExpandResult < T > = ValueResult < T , ExpandError > ;
    };
}

ExpandResult!()