macro_rules! deps {
    () => {
        NoNI!();
        SseMachine!();
        NoS4!();
        NoS3!();
    };
}

macro_rules! SSE2 {
    () => {
        deps!();
        pub type SSE2 = SseMachine < NoS3 , NoS4 , NoNI > ;
    };
}

SSE2!()