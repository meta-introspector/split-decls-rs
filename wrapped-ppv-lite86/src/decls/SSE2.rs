macro_rules! deps {
    () => {
        NoS4!();
        NoNI!();
        NoS3!();
        SseMachine!();
    };
}

macro_rules! SSE2 {
    () => {
        deps!();
        pub type SSE2 = SseMachine < NoS3 , NoS4 , NoNI > ;
    };
}

SSE2!();