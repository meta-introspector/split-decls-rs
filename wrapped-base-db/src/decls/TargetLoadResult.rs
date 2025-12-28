macro_rules! deps {
    () => {
        TargetData!();
        TargetLoadError!();
    };
}

macro_rules! TargetLoadResult {
    () => {
        deps!();
        pub type TargetLoadResult = Result < TargetData , TargetLoadError > ;
    };
}

TargetLoadResult!();