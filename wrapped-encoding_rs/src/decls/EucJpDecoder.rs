macro_rules! deps {
    () => {
        EucJpPending!();
    };
}

macro_rules! EucJpDecoder {
    () => {
        deps!();
        pub struct EucJpDecoder { pending : EucJpPending , }
    };
}

EucJpDecoder!();