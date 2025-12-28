macro_rules! deps {
    () => {
        YesS3!();
        SseMachine!();
        NoNI!();
        YesS4!();
    };
}

macro_rules! SSE41 {
    () => {
        deps!();
        pub type SSE41 = SseMachine < YesS3 , YesS4 , NoNI > ;
    };
}

SSE41!()