macro_rules! deps {
    () => {
        YesS3!();
        YesS4!();
        SseMachine!();
        NoNI!();
        AVX2!();
    };
}

macro_rules! AVX {
    () => {
        deps!();
        # [doc = " AVX but not AVX2: only 128-bit integer operations, but use VEX versions of everything"] # [doc = " to avoid expensive SSE/VEX conflicts."] pub type AVX = SseMachine < YesS3 , YesS4 , NoNI > ;
    };
}

AVX!()