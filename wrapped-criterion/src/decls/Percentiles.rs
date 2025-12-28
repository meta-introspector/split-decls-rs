macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! Percentiles {
    () => {
        deps!();
        # [doc = " A \"view\" into the percentiles of a sample"] pub struct Percentiles < A > (Box < [A] >) where A : Float ;
    };
}

Percentiles!()