macro_rules! deps {
    () => {
        TargetLoadError!();
        TargetData!();
    };
}

macro_rules! TargetLoadResult {
    () => {
        deps!();
        pub type TargetLoadResult = Result < TargetData , TargetLoadError > ;
    };
}

TargetLoadResult!()