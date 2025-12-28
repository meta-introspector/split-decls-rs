macro_rules! deps {
    () => {
        NoS4!();
        NoNI!();
        SseMachine!();
        YesS3!();
    };
}

macro_rules! SSSE3 {
    () => {
        deps!();
        pub type SSSE3 = SseMachine < YesS3 , NoS4 , NoNI > ;
    };
}

SSSE3!()