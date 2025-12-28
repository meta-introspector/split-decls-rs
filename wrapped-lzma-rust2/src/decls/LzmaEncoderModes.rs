macro_rules! deps {
    () => {
        NormalEncoderMode!();
        FastEncoderMode!();
    };
}

macro_rules! LzmaEncoderModes {
    () => {
        deps!();
        pub (crate) enum LzmaEncoderModes { Fast (FastEncoderMode) , Normal (NormalEncoderMode) , }
    };
}

LzmaEncoderModes!();