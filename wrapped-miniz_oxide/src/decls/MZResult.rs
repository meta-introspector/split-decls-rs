macro_rules! deps {
    () => {
        MZStatus!();
        MZError!();
        Result!();
    };
}

macro_rules! MZResult {
    () => {
        deps!();
        # [doc = " `Result` alias for all miniz status codes both successful and failed."] pub type MZResult = Result < MZStatus , MZError > ;
    };
}

MZResult!();