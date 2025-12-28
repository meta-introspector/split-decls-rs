macro_rules! deps {
    () => {
        TargetData!();
    };
}

macro_rules! Target {
    () => {
        deps!();
        pub type Target = Idx < TargetData > ;
    };
}

Target!()