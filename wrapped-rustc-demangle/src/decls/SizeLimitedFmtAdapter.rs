macro_rules! deps {
    () => {
        SizeLimitExhausted!();
    };
}

macro_rules! SizeLimitedFmtAdapter {
    () => {
        deps!();
        struct SizeLimitedFmtAdapter < F > { remaining : Result < usize , SizeLimitExhausted > , inner : F , }
    };
}

SizeLimitedFmtAdapter!();