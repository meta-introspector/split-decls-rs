macro_rules! deps {
    () => {
        YesS4!();
        NoNI!();
        YesS3!();
        AVX2!();
        SseMachine!();
    };
}

macro_rules! AVX {
    () => {
        deps!();
        # [doc = " AVX but not AVX2: only 128-bit integer operations, but use VEX versions of everything"] # [doc = " to avoid expensive SSE/VEX conflicts."] pub type AVX = SseMachine < YesS3 , YesS4 , NoNI > ;
    };
}

AVX!();