macro_rules! deps {
    () => {
        NaiveConfig!();
    };
}

macro_rules! Naive {
    () => {
        deps!();
        # [doc = " Comparatively simple implementation that can be used as something to compare against in tests"] pub struct Naive { encode_table : [u8 ; 64] , decode_table : [u8 ; 256] , config : NaiveConfig , }
    };
}

Naive!()